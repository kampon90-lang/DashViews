use dioxus::{html::div, prelude::*};

// Header of each card
#[derive(Props, Clone, PartialEq)]
pub struct StatProps {
    title: String,
    value: String,
    trend: String,
    #[props(default = "text-green-500".to_string())]
    trend_color: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatCardVariant {
    Sky,
    Slate,
}

#[component]
pub fn StatCard(
    label: &'static str,
    value: &'static str,
    hint: &'static str,
    icon: &'static str,
    #[props(default = StatCardVariant::Slate)] variant: StatCardVariant,
) -> Element {
    let accent = match variant {
        StatCardVariant::Sky => "border-sky-500/50 bg-sky-500/10",
        StatCardVariant::Slate => "border-slate-800 bg-slate-900/50",
    };

    rsx! {
        div {
            class: accent,

            div { class: "flex items-start justify-between gap-3",
                div { class: "min-w-0",
                    div { class: "card-title", "{label}"}
                    div {class: "mt-2 text-2xl font-semibold text-slate-100", "{value}"}
                    div {class: "card-sub", "{hint}"}
                } // 1st end 2nd inner div
            } // end 1st inner div

            div { class: "h-10 w-10 rounded-2xl border border-slate-700/70 bg-slate-950/30 flex items-center justify-center",
                span {class: "text-lg", "{icon}"}
            } // 2nd of 1st inner div
        } // end outter div
    } // end rsx!
} // end function here
