use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8210")?
        .run()
        .await
}

pub async fn send_request() -> anyhow::Result<String> {
    let body = reqwest::get("http://127.0.0.1:8210").await?.text().await?;

    Ok(body)
}
