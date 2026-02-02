// import library

use dioxus::prelude::*;

// import module components

mod components;
mod hero;
mod pages;

// use modules
use hero::Hero;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
} // close main

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}
    }
} // close app element
