use std::net::{Ipv4Addr, SocketAddr};

use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_http_common::{
    middleware::redirect::redirect_middleware,
    response::{HttpApiResponse, RedirectResponse, StatusResponse},
};
use poem_openapi::{payload::Json, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/index", method = "get")]
    async fn test(&self) -> HttpApiResponse<StatusResponse> {
        let status = StatusResponse { status: true };
        HttpApiResponse::Ok(Json(status))
    }

    #[oai(path = "/redirect", method = "get")]
    async fn redirect(&self) -> HttpApiResponse<StatusResponse> {
        HttpApiResponse::MovedPermanently(Json(RedirectResponse {
            url: "https://www.google.com".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_service = OpenApiService::new(Api, "test", "v1").server(format!("http://localhost:8080"));
    // let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest("/", api_service)
        // .nest("/ui", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .around(redirect_middleware);

    let _ = Server::new(TcpListener::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080)))
        .run(route)
        .await;
    Ok(())
}
