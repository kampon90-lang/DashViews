use crate::components::sidebar::NavKey;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BottomNavProbs {
    pub active: Signal<NavKey>,
}

#[component]
pub fn BottomNav(props: BottomNavProbs) -> Element {
    let mut active = props.active;

    let item = |key: NavKey, icon: &'static str, label: &'static str| {
        let is_active = *active.read() == key;
        let cls = if is_active {
            "bottom-nav-item bottom-nav-item-active"
        } else {
            "bottom-nav-item"
        };

        rsx! {
            button {
                class: "{cls}",
                onclick: move |_| active.set(key),

                div { class: "bottom-nav-icon", "{icon}"}
                div { class: "bottom-nav-label", "{label}"}
            }
        }
    };

    rsx! {
        // md: hidden
        nav { class: "bottom-nav md:hidden",
            {item(NavKey::DashBoard, "📊", "Home")}
            {item(NavKey::Rooms, "🏠", "Rooms")}
            {item(NavKey::Tenants, "👤", "Tenants")}
            {item(NavKey::Bills, "🧾", "Bills")}
            {item(NavKey::Settings, "⚙️", "Settings")}

        }
    }
}
