// import library

use dioxus::html::g::class;
use dioxus::prelude::*;

// import module components

mod components;

// use modules

use crate::components::sidebar::*;
use crate::components::stat_card::*;

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

#[component]
pub fn Hero() -> Element {
    rsx! {
        div
        { class: "w-screen h-screen flex flex-col",
            div
            { class: "h-[5rem] bg-red-600",

            } // div narbar

            div
            { class: "flex flex-1",
                div
                { class: "w-[12rem] bg-blue-900",

                } // div sidebar

                div
                {  class: "flex-1 bg-black",

                } // div dash content

            } // div body content
        } // whole div

    } //close rsx! Hero
} // close Hero function
