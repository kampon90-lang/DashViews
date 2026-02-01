use dioxus::prelude::*;

#[component]
pub fn SideBar() -> Element {
    rsx! {
        div
        { class: "w-[250px] h-full bg-red-400",
            "this is side bar"
        }
    }
}
