use crate::models::CreateLinkModel;

pub enum Api {
    CreateLink(CreateLinkModel),
    GetAllLinks,
    OpenLink(String),
    NotFound,
    BadRequest,
}

impl From<spin_sdk::http::Request> for Api {
    fn from(value: spin_sdk::http::Request) -> Self {
        match value.method() {
            &http::Method::POST => match value.headers().get("spin-path-info") {
                Some(path) => {
                    let path = path.to_str().unwrap();
                    if path.starts_with("/create") {
                        let Ok(model) = CreateLinkModel::from_bytes(&value.body().clone().unwrap()) else {
                                return Api::BadRequest;
                            };
                        return Api::CreateLink(model);
                    }
                    Api::NotFound
                }
                None => {
                    println!("No path info");
                    Api::NotFound
                }
            },
            &http::Method::GET => match value.headers().get("spin-path-info") {
                Some(path) => {
                    let path = path.to_str().unwrap();
                    if path == "/all" {
                        return Api::GetAllLinks;
                    }
                    let path = path.replace("/", "");
                    Api::OpenLink(path)
                }
                None => Api::NotFound,
            },
            _ => Api::NotFound,
        }
    }
}
