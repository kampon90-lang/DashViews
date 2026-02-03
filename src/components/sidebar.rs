use dioxus::{html::track::label, prelude::*};

#[derive(Clone, Copy, PartialEq)]
pub enum NavKey {
    DashBoard,
    Rooms,
    Tenants,
    Bills,
    Settings,
}
#[component]
pub fn SideBar(collapsed: Signal<bool>, active: Signal<NavKey>) -> Element {
    let is_collapsed = *collapsed.read();
    let nav_item = |key: NavKey, icon: &'static str, text: &'static str| {
        let is_active = *active.read() == key;
        let base = if is_active {
            "nav-item nav-item-active"
        } else {
            "nav-item"
        };

        rsx! {
            button {
                class: "{base}",
                onclick: move |_| active.set(key),

                span { class: "nav-icon", "{icon}"}
                if !is_collapsed {
                    span { class: "nav-label", "{text}"}
                }
            }
        }
    };

    rsx! {
        aside { class: if is_collapsed {"sidebar sidebar-collapsed hidden md:flex"} else {"sidebar hidden md:flex"},
            // Brand
            div { class: "sidebar-brand",
                div { class: "brand-mark", "🏠"}
                if !is_collapsed {
                    div {
                        div { class: "text-base font-semibold text-slate-100", "Rental"}
                        div { class: "text-xs text-slate-400", "Manage"}
                    }
                }
            }
        }

        // Collapse control
        div { class: "mt-3",
            button {
                class: "btn w-full justify-center",
                onclick: move |_|{
                    let v = *collapsed.read();
                    collapsed.set(!v);
                },
                if is_collapsed {">>"} else {"<<"}
            }

        }

        // Nav
        nav { class: "sidebar-nav mt-4",
            {nav_item(NavKey::DashBoard, "📊", "Dashboard")}
            {nav_item(NavKey::Rooms, "🏘️", "Rooms")}
            {nav_item(NavKey::Tenants, "🧾", "Bills")}
            {nav_item(NavKey::Settings, "⚙️", "Settings")}

        }

        // footer
        if !is_collapsed {
            div { class: "sidebar-footer",
                div { class: "text-xs text-slate-400",
                    "Blue/Slate tone: buit with Dioxus 0.7 + Tailwind"
                }
            }
        }
    }
}
