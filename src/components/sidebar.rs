use dioxus::prelude::*;

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
    let brand = if is_collapsed { "RM" } else { "Rental Manager" };

    rsx! {
        aside {
            class: "panel-solid h-[calc(100vh-16px) m-2 p-3 w-[86px] sm:w-[92px]",
            class: if !is_collapsed {"lg:w-[270px]"} else {"lg:w-[86px]"},

            div { class: "flex items-center justify-between gap-2 px-2 py-2",
                div { class: "flex items-center gap-3",
                    div { class: "h-10 w-10 rounded-2xl bg-sky-500/15 border border-sky-500/20 flex items-center justify-center",
                        span { class: "text-sky-100 font-bold",
                            "🏠"
                        } // end span
                    } // end 3rd inner div
                } // end 2nd inner div
            } // end 1st div

            button { class: "btn btn-ghost hidden lg:inline-flex",
                onclick: move |_| collapsed.set(!is_collapsed),
                if is_collapsed {">>"} else {"<<"}

            } // end

            div { class: "line-divider my-3" }

            nav { class: "flex flex-col gap-1",

                NavItem {collapsed, active, nav_id: NavKey::DashBoard, icon: "📊", label: "Dashboard"}
                NavItem {collapsed, active, nav_id: NavKey::Rooms, icon: "🚪", label: "Rooms"}
                NavItem {collapsed, active, nav_id: NavKey::Tenants, icon: "👤", label: "Tenants"}
                NavItem {collapsed, active, nav_id: NavKey::Bills, icon: "🧾", label: "Bills"}
                NavItem {collapsed, active, nav_id: NavKey::Settings, icon: "⚙️", label: "Settings"}

            } // end nav element

            div {class: "mt-auto pt-4"}
            div { class: "line-divider my-3"}

            div { class: "px-2 pb-1",
                div { class: "badge",
                    "Running on RPi: 192.168.1.110"
                } // end inner div

                if !is_collapsed {
                    div { class: "mt-2 text-xs text-slate-500",
                        "Blue/Slate tone: built with Dioxus 0.7 + Tailwind"
                    }
                } // end if
            } // end outter div
        }
    }
}

#[component]
fn NavItem(
    collapsed: Signal<bool>,
    active: Signal<NavKey>,
    nav_id: NavKey,
    icon: &'static str,
    label: &'static str,
) -> Element {
    let is_collapsed = *collapsed.read();
    let is_active = *active.read() == nav_id;

    rsx! {
        a {
            href: "#",
            class: "nav-item",
            class: if is_active {"nav-item-active"} else {""},

            onclick: move |_| active.set(nav_id),

            span {class: "text-base", "{icon}"}

            if !is_collapsed {
                span {class: "truncate", "{label}"}
            }
        } // end a element
    } // end rsx!
} // end function
