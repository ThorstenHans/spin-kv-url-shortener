use std::collections::HashMap;

use anyhow::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use spin_sdk::key_value::Store;

use crate::models::{CreateLinkModel, LinkCreatedModel};

pub(crate) fn handle_create_short_link(
    store: &Store,
    model: CreateLinkModel,
) -> Result<LinkCreatedModel> {
    let mut short = get_short();
    let mut exists = store.exists(&short)?;
    while exists {
        short = get_short();
        exists = store.exists(&short)?;
    }

    store.set(&short, &model.url).unwrap();
    Ok(LinkCreatedModel {
        short,
        url: model.url,
    })
}

pub(crate) fn handle_open_short_link(store: &Store, short: String) -> Option<String> {
    match store.exists(&short) {
        Ok(exists) => {
            if !exists {
                return None;
            }
            let Ok(url) = store.get(&short) else {
                return None;
            };
            Some(String::from_utf8(url).unwrap())
        }
        Err(_) => None,
    }
}

pub(crate) fn handle_get_all_short_links(store: &Store) -> Result<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    let keys = store.get_keys()?;
    for key in keys {
        let value = store.get(&key)?;
        let value = String::from_utf8(value)?;
        map.insert(key, value);
    }
    Ok(map)
}

fn get_short() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}
