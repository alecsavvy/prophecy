use anyhow::Result;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::Json, OpenApi, OpenApiService};
use serde_json::{json, Value};

use crate::config::Config;

struct WebServer;

#[OpenApi]
impl WebServer {
    #[oai(path = "/health_check", method = "get")]
    async fn health_check(&self) -> Json<Value> {
        Json(json!({
            "status": "up"
        }))
    }
}

pub async fn web_server(config: Config) -> Result<()> {
    let addr = config.web_server_addr;
    let api_service = OpenApiService::new(WebServer, "Prophecy", "1.0");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);
    let tcp_listener = TcpListener::bind(addr);
    Server::new(tcp_listener).run(app).await?;
    Ok(())
}
