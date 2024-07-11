#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

use homepage::{
    background::component::Background, footer::component::Footer, navbar::component::Navbar,
};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/home")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Home {})]
    Home {}
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    Background::render();

    rsx! {
        Navbar {}
        Footer {}
    }
}
