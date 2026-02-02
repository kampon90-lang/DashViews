use dioxus::prelude::*;

#[component]
pub fn TenantsPage() -> Element {
    rsx! {
        div { class: "page card-pad",
            div { class: "card-title", "Tenants"}
            div { class: "muted mt-1", "Coming soon..."}
        }
    }
}
