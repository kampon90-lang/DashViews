use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TopBarProps {
    pub title: &'static str,
}

#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    rsx! {
        header { class: "sticky top-0 z-20 backdrop-blur border-b border-slate-800/80",
            div { class: "mx-2 mt-2",
                div { class: "surface p-4 flex items-center justify-between gap-3",

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
                        div { class: "top-bar-res",
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
