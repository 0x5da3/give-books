use crate::consts::*;
use crate::structs::*;
use dioxus::prelude::*;

use scraper::{Html, Selector};

pub fn parse_json(input_url: Signal<String>, mut v_ogp: Signal<Vec<OGP>>) {
    if input_url.read().is_empty() {
        tracing::info!("url is null");
        return;
    }

    spawn(async move {
        let resp = reqwest::get(input_url.to_string())
            .await
            .unwrap()
            .text()
            .await;

        match resp {
            Ok(data) => {
                let fragment = Html::parse_fragment(&data);
                let meta = Selector::parse("meta").unwrap();
                let mut og_title = String::new();
                let mut og_url = String::new();
                let mut og_image = String::new();
                let mut og_desc = String::new();

                for el in fragment.select(&meta) {
                    match el.value().attr("property") {
                        Some(attr) => match attr {
                            OG_DESC => og_desc = el.attr("content").unwrap().to_string(),
                            OG_IMAGE => og_image = el.attr("content").unwrap().to_string(),
                            OG_TITLE => og_title = el.attr("content").unwrap().to_string(),
                            OG_URL => og_url = el.attr("content").unwrap().to_string(),
                            _ => tracing::info!("not {:?}", el.value()),
                        },
                        _ => (),
                    }
                }
                let ogp = OGP {
                    og_desc,
                    og_image,
                    og_title,
                    og_url,
                };

                v_ogp.push(ogp.clone());
                let serialized: String = serde_json::to_string(&ogp).unwrap();
                tracing::info!("{}", serialized);
            }
            Err(err) => {
                tracing::info!("ng {err}");
            }
        }
    });
}

pub fn get_vogp() -> VOgp {
    let deserialized: VOgp = serde_json::from_str(JSON_OGP).unwrap();
    tracing::info!("{deserialized:?}");
    deserialized
}
