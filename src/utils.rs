use crate::consts::*;
use crate::structs::*;
use dioxus::prelude::*;

use scraper::{Html, Selector};

/// Returns `url` only if it uses a safe `http`/`https` scheme, otherwise an
/// empty string. OGP data comes from untrusted scraped/uploaded content, and
/// Dioxus escapes HTML entities but does NOT block dangerous URL schemes, so
/// without this an attacker could inject `javascript:`/`data:` URIs into
/// `<a href>` / `<img src>` (XSS).
pub fn sanitize_url(url: &str) -> String {
    let trimmed = url.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("https://") || lower.starts_with("http://") {
        trimmed.to_string()
    } else {
        if !trimmed.is_empty() {
            tracing::warn!("blocked potentially unsafe url");
        }
        String::new()
    }
}

pub fn sanitize_ogp(ogp: &OGP) -> OGP {
    OGP {
        og_title: ogp.og_title.clone(),
        og_desc: ogp.og_desc.clone(),
        og_url: sanitize_url(&ogp.og_url),
        og_image: sanitize_url(&ogp.og_image),
    }
}

pub fn sanitize_vogp(vogp: VOgp) -> VOgp {
    VOgp {
        records: vogp.records.iter().map(sanitize_ogp).collect(),
    }
}

pub fn parse_json(input_url: Signal<String>, mut v_ogp: Signal<Vec<OGP>>) {
    if input_url.read().is_empty() {
        tracing::info!("url is null");
        return;
    }

    spawn(async move {
        let response = match reqwest::get(input_url.to_string()).await {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("request failed: {err}");
                return;
            }
        };

        match response.text().await {
            Ok(data) => {
                let fragment = Html::parse_fragment(&data);
                let meta = Selector::parse("meta").unwrap();
                let mut og_title = String::new();
                let mut og_url = String::new();
                let mut og_image = String::new();
                let mut og_desc = String::new();

                for el in fragment.select(&meta) {
                    if let Some(attr) = el.value().attr("property") {
                        let content = el.attr("content").unwrap_or_default().to_string();
                        match attr {
                            OG_DESC => og_desc = content,
                            OG_IMAGE => og_image = content,
                            OG_TITLE => og_title = content,
                            OG_URL => og_url = content,
                            _ => tracing::info!("not {:?}", el.value()),
                        }
                    }
                }
                let ogp = sanitize_ogp(&OGP {
                    og_desc,
                    og_image,
                    og_title,
                    og_url,
                });

                v_ogp.push(ogp.clone());
                let serialized: String = serde_json::to_string(&ogp).unwrap_or_default();
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
    let sanitized = sanitize_vogp(deserialized);
    tracing::info!("{sanitized:?}");
    sanitized
}
