use dioxus::prelude::*;

#[component]
pub fn RoomsPage() -> Element {
    rsx! {
        div { class: "page card-pad",
            div { class: "card-title", "Rooms"}
            div { class: "muted mt-1", "Coming soon..."}
        }
    }
}
