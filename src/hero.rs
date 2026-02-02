use dioxus::prelude::*;

// import module components

// use modules
use crate::components::sidebar::NavKey;
use crate::components::{SideBar, TopBar};
use crate::pages::dashboard::DashBoardPage;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Hero() -> Element {
    let collapsed = use_signal(|| false);
    let active = use_signal(|| NavKey::DashBoard);

    rsx! {
        div { class: "app-shell",
            div { class: "app-row",
                // sidebar
                SideBar { collapsed, active }

                // Main area
                div { class: "min-col",
                    TopBar {title: title_of(*active.read())}

                    // page contents
                    div { class: "container-page",
                        match *active.read() {
                            NavKey::DashBoard => rsx!{ crate::pages::dashboard::DashBoardPage {}},
                            NavKey::Rooms => rsx! { crate::pages::rooms::RoomsPage{}},
                            NavKey::Tenants => rsx!{ crate::pages::tenants::TenantsPage {}},
                            NavKey::Bills => rsx!{ crate::pages::bills::BillsPage {}},
                            NavKey::Settings => rsx!{ crate::pages::settings::SettingsPage {}},
                        }

                    }
                }
            }
        }
    }
} // close Hero function

#[component]
fn Placeholder(title: &'static str) -> Element {
    rsx! {
        main { class: "container-page py-9",
            div { class: "card bg-lineart",
                div { class: "text-xl font-semibold", "{title}"}
                div { class: "mt-2 text-sm text-slate-400",
                    "Todo!!"
                }
            }
        }
    }
}

// Helper
fn title_of(id_key: NavKey) -> &'static str {
    match id_key {
        NavKey::DashBoard => "DashBoard",
        NavKey::Rooms => "Rooms",
        NavKey::Tenants => "Tenants",
        NavKey::Bills => "Bills",
        NavKey::Settings => "Settings",
    }
}
