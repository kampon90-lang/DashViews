use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TopBarProps {
    pub title: &'static str,
}

#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    rsx! {
        header { class: "sticky top-0 z-20 backdrop-blur shadow-[0_1px_0_rgba(255,255,255,0.05)] border-slate-800/80",
            div { class: "px-3 sm:px-4 pt-3",
                div { class: "surface p-3 sm:p-4 flex flex-col md:flex-row md:items-center md:justify-between gap-3",

                    // Left: title and breadcrumbs
                    div{ class: "min-w-0",
                        div { class: "text-sm font-semibold text-slate-100 truncate",
                            "{props.title}"
                        }

                        div { class: "mt-1 text-xs text-slate-400 truncate",
                            "Rental preview | Income | Vacant | Unpaid"
                        }
                    }

                    // Right search + action
                    div { class: "flex items-center gap-2 flex-wrap justify-end",
                        // Search bar
                        div { class: "hidden md:block flex-1 min-w-0 w-72 lg:w-96",
                            input {
                                class: "input",
                                r#type: "text",
                                placeholder: "Seach Tenant/Room/Bill"
                            }
                        }
                        button { class: "btn btn-primary", "New Bill" }
                        button { class: "btn", "Export" }


                    }


                }
            }
        }
    } // end rsx!
}
