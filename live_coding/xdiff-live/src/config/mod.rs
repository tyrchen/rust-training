mod xdiff;
mod xreq;

use crate::ExtraArgs;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    Client, Method, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::fmt::Write;
use std::str::FromStr;
use tokio::fs;
use url::Url;

pub use xdiff::{DiffConfig, DiffProfile, ResponseProfile};
pub use xreq::RequestConfig;

#[async_trait]
pub trait LoadConfig
where
    Self: Sized + ValidateConfig + DeserializeOwned,
{
    /// load config from yaml file
    async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    /// load config from yaml string
    fn from_yaml(content: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }
}

pub trait ValidateConfig {
    fn validate(&self) -> Result<()>;
}

pub fn is_default<T: Default + PartialEq>(v: &T) -> bool {
    v == &T::default()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "empty_json_value", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct ResponseExt(Response);

fn empty_json_value(v: &Option<serde_json::Value>) -> bool {
    v.as_ref().map_or(true, |v| {
        v.is_null() || (v.is_object() && v.as_object().unwrap().is_empty())
    })
}

impl RequestProfile {
    pub fn new(
        method: Method,
        url: Url,
        params: Option<serde_json::Value>,
        headers: HeaderMap,
        body: Option<serde_json::Value>,
    ) -> Self {
        Self {
            method,
            url,
            params,
            headers,
            body,
        }
    }

    pub async fn send(&self, args: &ExtraArgs) -> Result<ResponseExt> {
        let (headers, query, body) = self.generate(args)?;
        let client = Client::new();
        let req = client
            .request(self.method.clone(), self.url.clone())
            .query(&query)
            .headers(headers)
            .body(body)
            .build()?;

        let res = client.execute(req).await?;
        Ok(ResponseExt(res))
    }

    pub fn get_url(&self, args: &ExtraArgs) -> Result<String> {
        let mut url = self.url.clone();
        let (_, params, _) = self.generate(args)?;

        if !params.as_object().unwrap().is_empty() {
            let query = serde_qs::to_string(&params)?;
            url.set_query(Some(&query));
        }
        Ok(url.to_string())
    }

    fn generate(&self, args: &ExtraArgs) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| json!({}));

        for (k, v) in &args.headers {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        for (k, v) in &args.query {
            query[k] = v.parse()?;
        }

        for (k, v) in &args.body {
            body[k] = v.parse()?;
        }

        let content_type = get_content_type(&headers);
        match content_type.as_deref() {
            Some("application/json") => {
                let body = serde_json::to_string(&body)?;
                Ok((headers, query, body))
            }
            Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
                let body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, body))
            }
            _ => Err(anyhow::anyhow!("unsupported content-type")),
        }
    }
}

impl ValidateConfig for RequestProfile {
    fn validate(&self) -> Result<()> {
        if let Some(params) = self.params.as_ref() {
            if !params.is_object() {
                return Err(anyhow::anyhow!(
                    "Params must be an object but got\n{}",
                    serde_yaml::to_string(params)?
                ));
            }
        }
        if let Some(body) = self.body.as_ref() {
            if !body.is_object() {
                return Err(anyhow::anyhow!(
                    "Body must be an object but got\n{}",
                    serde_yaml::to_string(body)?
                ));
            }
        }
        Ok(())
    }
}

impl ResponseExt {
    pub fn into_inner(self) -> Response {
        self.0
    }

    pub async fn get_text(self, profile: &ResponseProfile) -> Result<String> {
        let res = self.0;
        let mut output = get_status_text(&res)?;

        write!(
            &mut output,
            "{}",
            get_header_text(&res, &profile.skip_headers)?
        )?;

        write!(
            &mut output,
            "{}",
            get_body_text(res, &profile.skip_body).await?
        )?;

        Ok(output)
    }

    pub fn get_header_keys(&self) -> Vec<String> {
        let res = &self.0;
        let headers = res.headers();
        headers
            .iter()
            .map(|(k, _)| k.as_str().to_string())
            .collect()
    }
}

pub fn get_status_text(res: &Response) -> Result<String> {
    Ok(format!("{:?} {}\n", res.version(), res.status()))
}

pub fn get_header_text(res: &Response, skip_headers: &[String]) -> Result<String> {
    let mut output = String::new();

    let headers = res.headers();
    for (k, v) in headers.iter() {
        if !skip_headers.iter().any(|sh| sh == k.as_str()) {
            writeln!(&mut output, "{}: {:?}", k, v)?;
        }
    }
    writeln!(&mut output)?;

    Ok(output)
}

pub async fn get_body_text(res: Response, skip_body: &[String]) -> Result<String> {
    let content_type = get_content_type(res.headers());
    let text = res.text().await?;

    match content_type.as_deref() {
        Some("application/json") => filter_json(&text, skip_body),
        _ => Ok(text),
    }
}

fn filter_json(text: &str, skip: &[String]) -> Result<String> {
    let mut json: serde_json::Value = serde_json::from_str(text)?;

    // For now we just ignore non-object values, we don't know how to filter.
    // In future, we might support array of objects
    if let serde_json::Value::Object(ref mut obj) = json {
        for k in skip {
            obj.remove(k);
        }
    }

    Ok(serde_json::to_string_pretty(&json)?)
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().unwrap().split(';').next().map(|v| v.to_string()))
}

impl FromStr for RequestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut url = Url::parse(s)?;
        let qs = url.query_pairs();
        let mut params = json!({});
        for (k, v) in qs {
            params[&*k] = v.parse()?;
        }
        url.set_query(None);

        Ok(RequestProfile::new(
            Method::GET,
            url,
            Some(params),
            HeaderMap::new(),
            None,
        ))
    }
}

#[cfg(test)]
mod tests {
    use mockito::{mock, Mock};
    use reqwest::StatusCode;

    use super::*;

    #[tokio::test]
    async fn request_profile_send_should_work() {
        let _m = mock_for_url("/todo?a=1&b=2", json!({"id": 1, "title": "todo"}));

        let res = get_response("/todo?a=1&b=2", &Default::default())
            .await
            .into_inner();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn request_profile_send_with_extra_args_should_work() {
        let _m = mock_for_url("/todo?a=1&b=3", json!({"id": 1, "title": "todo"}));

        let args = ExtraArgs::new_with_query(vec![("b".into(), "3".into())]);

        let res = get_response("/todo?a=1&b=2", &args).await.into_inner();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    fn request_profile_get_url_should_work() {
        let profile = get_profile("/todo?a=1&b=2");
        assert_eq!(
            profile.get_url(&Default::default()).unwrap(),
            get_url("/todo?a=1&b=2")
        );
    }

    #[test]
    fn request_profile_get_url_with_args_should_work() {
        let profile = get_profile("/todo?a=1&b=2");
        let args = ExtraArgs::new_with_query(vec![("c".into(), "3".into())]);
        assert_eq!(
            profile.get_url(&args).unwrap(),
            get_url("/todo?a=1&b=2&c=3")
        );
    }

    #[tokio::test]
    async fn get_status_text_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));

        let res = get_response("/todo", &Default::default())
            .await
            .into_inner();
        assert_eq!(get_status_text(&res).unwrap(), "HTTP/1.1 200 OK\n");
    }

    #[tokio::test]
    async fn get_header_text_shoud_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));

        let res = get_response("/todo", &Default::default())
            .await
            .into_inner();
        assert_eq!(
            get_header_text(&res, &["connection".into(), "content-length".into()]).unwrap(),
            "content-type: \"application/json\"\n\n"
        );
    }

    #[tokio::test]
    async fn get_body_text_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));

        let res = get_response("/todo", &Default::default())
            .await
            .into_inner();
        assert_eq!(
            get_body_text(res, &["id".into()]).await.unwrap(),
            "{\n  \"title\": \"todo\"\n}"
        );
    }

    #[test]
    fn request_profile_validate_should_work() {
        let profile = get_profile("/todo?a=1&b=2");
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn request_profile_with_bad_param_validate_should_fail() {
        let profile = RequestProfile::new(
            Method::GET,
            Url::parse("http://localhost:8080/todo").unwrap(),
            Some(json!([1, 2, 3])),
            HeaderMap::new(),
            None,
        );
        let result = profile.validate();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Params must be an object but got\n- 1\n- 2\n- 3\n"
        );
    }

    #[tokio::test]
    async fn response_ext_get_text_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));

        let res = get_response("/todo", &Default::default()).await;
        let response_profile = ResponseProfile::new(
            vec!["connection".into(), "content-length".into()],
            vec!["title".into()],
        );
        assert_eq!(
            res.get_text(&response_profile).await.unwrap(),
            "HTTP/1.1 200 OK\ncontent-type: \"application/json\"\n\n{\n  \"id\": 1\n}"
        );
    }

    #[tokio::test]
    async fn response_ext_get_header_keys_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));

        let res = get_response("/todo", &Default::default()).await;
        let mut sorted_header_keys = res.get_header_keys();
        sorted_header_keys.sort();
        let mut expected_header_keys = vec!["content-length", "content-type", "connection"];
        expected_header_keys.sort();
        assert_eq!(sorted_header_keys, expected_header_keys);
    }

    #[test]
    fn test_get_content_type() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        assert_eq!(
            get_content_type(&headers),
            Some("application/json".to_string())
        );
    }

    fn mock_for_url(path_and_query: &str, resp_body: serde_json::Value) -> Mock {
        mock("GET", path_and_query)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&resp_body).unwrap())
            .create()
    }

    fn get_url(path: &str) -> String {
        format!("{}{}", mockito::server_url(), path)
    }

    fn get_profile(path_and_query: &str) -> RequestProfile {
        let url = get_url(path_and_query);
        RequestProfile::from_str(&url).unwrap()
    }

    async fn get_response(path_and_query: &str, args: &ExtraArgs) -> ResponseExt {
        let profile = get_profile(path_and_query);
        profile.send(args).await.unwrap()
    }
}
