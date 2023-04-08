mod api;
mod handlers;
mod models;

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    key_value::Store,
};

use crate::{
    api::Api,
    handlers::{handle_create_short_link, handle_get_all_short_links, handle_open_short_link},
};

#[http_component]
fn handle_shortener(req: Request) -> Result<Response> {
    let r = Api::from(req);
    // create the "default" key-value store
    let store = Store::open_default()?;
    match r {
        Api::CreateLink(model) => {
            let res = handle_create_short_link(&store, model)?;
            let json = serde_json::to_string(&res)?;
            Ok(http::Response::builder()
                .status(http::StatusCode::CREATED)
                .body(Some(json.into()))?)
        }
        Api::GetAllLinks => {
            let Ok(map) = handle_get_all_short_links(&store) else {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(None)?);
            };
            let json = serde_json::to_string(&map)?;
            Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .body(Some(json.into()))?)
        }
        Api::OpenLink(short) => match handle_open_short_link(&store, short) {
            Some(url) => Ok(http::Response::builder()
                .status(http::StatusCode::PERMANENT_REDIRECT)
                .header(http::header::LOCATION, url)
                .body(None)?),
            None => Ok(http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(None)?),
        },
        Api::NotFound => Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?),
        Api::BadRequest => Ok(http::Response::builder()
            .status(http::StatusCode::BAD_REQUEST)
            .body(None)?),
    }
}
