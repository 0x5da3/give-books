#![allow(non_snake_case)]

pub mod components;
pub mod consts;
pub mod structs;
pub mod utils;

use crate::components::*;
use dioxus::prelude::*;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tracing::Level;

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
