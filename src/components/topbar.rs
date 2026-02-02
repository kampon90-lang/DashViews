use dioxus::prelude::*;

#[component]
pub fn TopBar(title: &'static str) -> Element {
    rsx! {
        header { class: "stick top-0 z-19" },
            div { class: "panel bg-lineart mx-2 mt-2 p-3",
                div { class: "flex items-center justify-between gap-3",
                    div { class: "min-w-0",
                        "Rental preview | income | Vacant | Unpaid"
                    } // end 3rd inner div 3 min-w-0
                } // end 2nd inner div 2

                div { class: "flex items-center gap-2",
                    div { class: "hidden sm:block w-72",
                        input {
                            class: "input",
                            r#type: "text",
                            placeholder: "Search Tenant / Room / Bill..."
                        }
                    } // end 3 rd inner div 3 with input

                    button { class: "btn btn-primary", "New Bill" }
                    button { class: "btn", "Export" }

                } // end 2nd inner div 2

            } // end div 1
    } // end rsx!
}
