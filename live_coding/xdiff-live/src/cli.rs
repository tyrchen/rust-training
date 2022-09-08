use crate::ExtraArgs;
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

/// Diff two http requests and compare the difference of the responses
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
#[non_exhaustive]
pub enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
    /// Parse URLs to generate a profile
    Parse,
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// Profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Overrides args. Could be used to override the query, headers and body of the request.
    /// For query params, use `-e key=value`.
    /// For headers, use `-e %key=value`.
    /// For body, use `-e @key=value`.
    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    /// Configuration to use.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');

    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair"))?
        .trim();
    let value = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair"))?
        .trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
                KeyValType::Body => body.push((arg.key, arg.value)),
            }
        }

        Self {
            headers,
            query,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vec_key_val_for_extra_args_should_work() {
        let args = vec![
            KeyVal {
                key_type: KeyValType::Header,
                key: "key1".to_string(),
                value: "value1".to_string(),
            },
            KeyVal {
                key_type: KeyValType::Query,
                key: "key2".to_string(),
                value: "value2".to_string(),
            },
            KeyVal {
                key_type: KeyValType::Body,
                key: "key3".to_string(),
                value: "value3".to_string(),
            },
        ];

        let extra_args = ExtraArgs::from(args);

        assert_eq!(
            extra_args,
            ExtraArgs {
                headers: vec![("key1".to_string(), "value1".to_string())],
                query: vec![("key2".to_string(), "value2".to_string())],
                body: vec![("key3".to_string(), "value3".to_string())],
            }
        );
    }

    #[test]
    fn parse_key_val_should_work() {
        let args = vec!["%key1=value1", "key2=value2", "@key3=value3", "key4=value4"];

        let key_vals = args
            .into_iter()
            .map(|arg| parse_key_val(arg))
            .collect::<Result<Vec<_>>>()
            .unwrap();

        assert_eq!(
            key_vals,
            vec![
                KeyVal {
                    key_type: KeyValType::Header,
                    key: "key1".to_string(),
                    value: "value1".to_string(),
                },
                KeyVal {
                    key_type: KeyValType::Query,
                    key: "key2".to_string(),
                    value: "value2".to_string(),
                },
                KeyVal {
                    key_type: KeyValType::Body,
                    key: "key3".to_string(),
                    value: "value3".to_string(),
                },
                KeyVal {
                    key_type: KeyValType::Query,
                    key: "key4".to_string(),
                    value: "value4".to_string(),
                },
            ]
        );
    }
}
