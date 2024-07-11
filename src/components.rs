use super::*;
use crate::consts::*;
use crate::structs::*;
use crate::utils::*;
use dioxus::prelude::*;
use std::{collections::HashMap, str::Chars};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Content {},
    #[route("/home")]
    #[redirect("/:.._segments",|_segments:Vec<String>|Route::LoginForm {})]
    LoginForm {},
}

#[component]
fn Carousel(vogp: Signal<VOgp>) -> Element {
    rsx! {
        div { class: "carousel",
            for o in *vogp.read().records {
                div {
                    img { src: "{o.og_image}", width: 200, height: 250 }
                }
            }
        }
    }
}

#[component]
fn Container(v: Signal<Vec<OGP>>, bids: Signal<HashMap<String, (usize, bool)>>) -> Element {
    rsx!(
        div { class: "container",
            for o in v.read().iter() {
                div { class: "card card-skin",
                    link { rel: "stylesheet", href: "{CSS_PATH}" }
                    div { class: "card_imgframe",
                        img { src: "{o.og_image}", width: 200, height: 250 }
                    }
                    div { class: "card_textbox",
                        if bids.read().get(&o.og_title).is_some() {
                            div { "{bids.read().get(&o.og_title).unwrap().0}件" }
                        } else {
                            div { "0件" }
                        }
                        div { class: "card_titletext",
                            // FIXME BadCode
                            if bids.read().get(&o.og_title).is_some() {
                                input {
                                    class: "card_checkbox",
                                    r#type: "checkbox",
                                    name: "{o.og_title}",
                                    value: "{bids.read().get(&o.og_title).unwrap().1}"
                                }
                            } else {
                                input {
                                    class: "card_checkbox",
                                    r#type: "checkbox",
                                    name: "{o.og_title}",
                                    value: "false"
                                }
                            }
                            a { href: "{o.og_url}", "{o.og_title}" }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn ContainerFromDB(mut bids: Signal<HashMap<String, (usize, bool)>>) -> Element {
    let future = use_resource(|| async move {
        let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();

        let _ = db
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await;

        let _ = db.use_ns("ns").use_db("testdb").await;

        let v: Vec<OGP> = db.select("ogp").await.unwrap();
        let signal_v = use_signal(|| v);
        signal_v
    });

    match &*future.read_unchecked() {
        Some(response) => {
            rsx! {
                form {
                    id: "t",
                    onsubmit: move |evt: Event<FormData>| {
                        let vs = evt.values();
                        if !vs.is_empty() {
                            tracing::info!("{vs:#?}");
                            for (title, _tupple) in vs {
                                bids.write()
                                    .entry(title.clone())
                                    .and_modify(|tupple| {
                                        tupple.0 += 1;
                                        tupple.1 = false;
                                    })
                                    .or_insert((1, false));
                            }
                        }
                    },
                    Container { v: *response, bids }
                }
                button { r#type: "submit", form: "t", "tes" }
            }
        }
        None => rsx! {
            div { "Loading..." }
        },
    }
}

//const OGP_JSON_FILE: &str = manganis::mg!(file("./test.json"));

#[component]
fn ContainerFromJson(mut bids: Signal<HashMap<String, (usize, bool)>>) -> Element {
    let vogp = use_signal(|| get_vogp());
    let v = use_signal(|| vogp.read().records.clone());

    rsx!(Container { v, bids })
}

#[component]
fn Content() -> Element {
    let bids = use_signal(|| HashMap::from([("".to_string(), (0, false))]));

    rsx! {
        if IS_BUILD_FOR_SSG {
            ContainerFromJson { bids }
        } else if !IS_UNDER_CONSTRUCTION {
            ContainerFromDB { bids }
        }
    }
}

#[component]
fn OGPParser() -> Element {
    let input_url = use_signal(|| "".to_string());
    let v_ogp = use_signal(|| vec![]);
    rsx!(
        if IS_PARSE_OGP {
            InputUrl { input_url }
            button { onclick: move |_| { parse_json(input_url, v_ogp) }, "..." }
        }
    )
}

#[component]
fn DatePicker() -> Element {
    rsx!(
        input { oninput: move |evt| { tracing::info!("{evt:?}") }, r#type: "date" }
        input { oninput: move |evt| { tracing::info!("{evt:?}") }, r#type: "date" }
    )
}

#[component]
fn _Dummy() -> Element {
    rsx!(Content {})
}

#[component]
fn File(vogp: Signal<VOgp>) -> Element {
    let mut filenames: Signal<Vec<String>> = use_signal(Vec::new);
    rsx! {
        input {
            r#type: "file",
            accept: ".json",
            multiple: false,
            onchange: move |evt: Event<FormData>| {
                async move {
                    if let Some(file_engine) = &evt.files() {
                        let files = file_engine.files();
                        for file_name in files {
                            if let Some(file) = file_engine.read_file_to_string(&file_name).await
                            {
                                let deserialized: VOgp = serde_json::from_str(&file).unwrap();
                                tracing::info!("{:?}", deserialized);
                                vogp.set(deserialized)
                            }
                            filenames.write().push(file_name);
                        }
                        tracing::info!("{:#?}", filenames);
                    }
                }
            }
        }
    }
}

#[component]
fn InputUrl(input_url: Signal<String>) -> Element {
    rsx! {
        div {
            div { "{input_url}" }
            form {
                onsubmit: move |event: Event<FormData>| {
                    for e in event.data.values() {
                        let (s, v) = e;
                        let vv = &v.0[0];
                        tracing::info!("Submitted! {s:?} {vv:?}");
                        let proxy = "https://corsproxy.io/?";
                        input_url.set(format!("{proxy}{vv}"))
                    }
                },
                input { name: "input_url2" }
                div {
                    input { r#type: "submit" }
                }
            }
        }
    }
}

#[component]
fn MailBox() -> Element {
    rsx!(
        div {
            input {
                r#type: "email",
                pattern: ".+@*",
                oninput: move |evt| { tracing::info!("ng {evt:?}") }
            }
        }
    )
}

#[component]
fn LoginBtn(is_disable_submit: Signal<bool>) -> Element {
    rsx!(input {
        onsubmit: move |evt| { tracing::info!("{evt:?}") },
        r#type: "submit",
        disabled: "{is_disable_submit}"
    })
}

#[component]
fn LoginForm() -> Element {
    let is_disable_submit = use_signal(|| true);
    let input_password = use_signal(|| String::from(""));

    rsx!(
        link { rel: "stylesheet", href: "main.css" }
        form {
            MailBox {}
            PasswordBox { input_password, is_disable_submit }
            LoginBtn { is_disable_submit }
        }
    )
}

/// FIXME Linkコンポーネントを使ってもページ更新(F5)しないと読み込まないので作った
/// to: Route::Countent{}
#[component]
fn MyLink(path: String, link_text: String) -> Element {
    rsx!(
        a { href: "{path}", "{link_text}" }
    )
}

#[component]
fn PasswordBox(input_password: Signal<String>, is_disable_submit: Signal<bool>) -> Element {
    let mut is_show_password = use_signal(|| false);
    let mut password_policy = use_signal(|| PasswordPolicy::new());

    rsx!(
        div {
            if *is_show_password.read() {
                input { r#type: "text", value: "{input_password}" }
            } else {
                input {
                    r#type: "password",
                    autocomplete: "new-password",
                    minlength: MIN_PASS_LENGTH as i64,
                    maxlength: 64,
                    required: true,
                    value: "{input_password}",
                    autocomplete: true,
                    oninput: move |evt: Event<FormData>| {
                        let mut p = PasswordPolicy::new();
                        let password = evt.value();
                        let chars: Chars = password.chars();
                        for c in chars.clone() {
                            tracing::info!("ng {c:?}");
                            if c.is_numeric() {
                                p.has_numeric = true;
                            } else if c.is_lowercase() {
                                p.has_lowercase = true;
                            } else if c.is_uppercase() {
                                p.has_uppercase = true;
                            }
                        }
                        if chars.count() > 7 {
                            p.has_enough_length = true;
                        }
                        password_policy.set(p.clone());
                        *input_password.write() = password;
                        if p.has_enough_length && p.has_lowercase && p.has_uppercase && p.has_numeric {
                            *is_disable_submit.write() = false;
                        } else {
                            *is_disable_submit.write() = true;
                        }
                    }
                }
            }
            input {
                r#type: "checkbox",
                onmouseup: move |_| { *is_show_password.write() = false },
                onmousedown: move |_| { *is_show_password.write() = true }
            }
            PasswordChecker { password_policy }
        }
    )
}

#[component]
fn PasswordChecker(password_policy: Signal<PasswordPolicy>) -> Element {
    rsx!(
        div {
            input {
                class: "password_checkbox",
                r#type: "checkbox",
                disabled: true,
                checked: password_policy.read().has_numeric
            }
            a { "数値" }
            input {
                class: "password_checkbox",
                r#type: "checkbox",
                disabled: true,
                checked: password_policy.read().has_lowercase
            }
            a { "英小文字" }
            input {
                class: "password_checkbox",
                r#type: "checkbox",
                disabled: true,
                checked: password_policy.read().has_uppercase
            }
            a { "英大文字" }
            input {
                class: "password_checkbox",
                r#type: "checkbox",
                disabled: true,
                checked: password_policy.read().has_enough_length
            }
            a { "{MIN_PASS_LENGTH}桁以上" }
        }
    )
}
