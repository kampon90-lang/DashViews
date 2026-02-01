use dioxus::prelude::*;

// Header of each card
#[derive(Props, Clone, PartialEq)]
pub struct StatProps {
    title: String,
    value: String,
    trend: String,
    #[props(default = "text-green-500".to_string())]
    trend_color: String,
}

#[component]
pub fn StatCard(props: StatProps) -> Element {
    rsx! {
        div
        { class: "stat-card-custom dorm-teal",
            p { class: "text-sm text-gray-500 uppercase", "{props.title}"}
            p { class: "text-2xl font-bold", "{props.value}"}
        }
    }
}
