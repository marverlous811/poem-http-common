use poem::{Endpoint, IntoResponse, Request, Response, Result};
use reqwest::StatusCode;

use crate::response::RedirectResponse;

pub async fn redirect_middleware<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    let res = next.call(req).await;
    match res {
        Ok(resp) => {
            let res = resp.into_response();
            if res.status() == 301 || res.status() == 302 {
                let body = res.into_body();
                match body.into_string().await {
                    Ok(body) => match serde_json::from_str::<RedirectResponse>(&body) {
                        Ok(redirect) => {
                            return Ok(Response::builder()
                                .status(StatusCode::MOVED_PERMANENTLY)
                                .header("Location", redirect.url)
                                .finish());
                        }
                        Err(err) => {
                            log::error!("Failed to parse redirect response: {}", err);
                            let res = Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(err.to_string());

                            return Ok(res);
                        }
                    },
                    Err(err) => {
                        log::error!("Failed to get body redirect response: {}", err);
                        let res = Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(err.to_string());

                        Ok(res)
                    }
                }
            } else {
                Ok(res)
            }
        }
        Err(err) => Err(err),
    }
}
