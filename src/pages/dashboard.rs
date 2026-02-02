use crate::components::{StatCard, StatCardVariant};
use dioxus::prelude::*;

#[component]
pub fn DashBoardPage() -> Element {
    rsx! {
        main { class: "container-page py-6",

            // KPI 4 ช่อง
            section { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4",
                StatCard {
                    label: "Monthly Revenue",
                    value: "124,500",
                    hint: "vs last month",
                    icon: "💰",
                    variant: StatCardVariant::Sky
                }
                StatCard {
                    label: "Occupied Rooms",
                    value: "8 / 10",
                    hint: "2 rooms vacant",
                    icon: "🏠",
                }
                StatCard {
                    label: "Outstanding Bills",
                    value: "3",
                    hint: "Due within 7 days",
                    icon: "🧾",
                }
                StatCard {
                    label: "Maintenance",
                    value: "1",
                    hint: "A203 Reported yesterday",
                    icon: "🛠️",
                }
            }

            // Summary + Recent activity (3 คอลัมน์)
            div { class: "mt-6 grid grid-cols-1 xl:grid-cols-3 gap-4",

                // LEFT (span 2): summary card
                div { class: "xl:col-span-2 card bg-lineart",
                    div { class: "flex items-center justify-between gap-3",
                        div {
                            div { class: "text-sm font-semibold text-slate-100", "Today" }
                            div { class: "mt-1 text-xs text-slate-400", "Quick action and highlight" }
                        }

                        div { class: "flex gap-2",
                            button { class: "btn btn-primary", "Create Bill" }
                            button { class: "btn", "Add Tenant" }
                            button { class: "btn", "Add Room" }
                        }
                    }

                    div { class: "line-divider my-4" }

                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-3",
                        SmallKpi { title: "Collected", value: "98000", note: "Promptpay + Cash" }
                        SmallKpi { title: "Expected", value: "126000", note: "Rental + Utilities" }
                        SmallKpi { title: "Net", value: "74000", note: "After Expenses" }
                    }
                }

                // RIGHT: recent activity
                div { class: "card",
                    div { class: "flex items-center justify-between",
                        div {
                            div { class: "text-sm font-semibold text-slate-100", "Recent activity" }
                            div { class: "mt-1 text-xs text-slate-400", "Latest Change" }
                        }
                        span { class: "badge", "Live" }
                    }

                    div { class: "line-divider my-4" }

                    ul { class: "space-y-3 text-sm",
                        ActivityRow { icon: "🧾", title:"Bill created", sub: "A101 | 6000 | 10:32" }
                        ActivityRow { icon: "💳", title:"Payment Received", sub: "B202 | 7100 | 10:32" }
                        ActivityRow { icon: "🛠️", title:"Maintenance", sub: "A203 | water leak | yesterday" }
                        ActivityRow { icon: "👤", title:"Tenant moved in", sub: "C301 | Contract signed" }
                    }
                }
            }

            // Room table preview
            div { class: "mt-6 card",
                div { class: "flex items-center justify-between gap-3",
                    div {
                        div { class: "text-sm font-semibold text-slate-100", "Room overview" }
                        div { class: "mt-1 text-xs text-slate-400", "Status snapshot (Preview)" }
                    }
                    button { class: "btn", "View all room" }
                }

                div { class: "line-divider my-4" }

                div { class: "overflow-x-auto",
                    table { class: "w-full text-sm",
                        thead { class: "text-slate-400",
                            tr {
                                th { class: "py-2 text-left font-medium", "Room" }
                                th { class: "py-2 text-left font-medium", "Type" }
                                th { class: "py-2 text-left font-medium", "Status" }
                                th { class: "py-2 text-left font-medium", "Rent" }
                                th { class: "py-2 text-left font-medium", "Tenant" }
                            }
                        }
                        tbody { class: "text-slate-200",
                            RoomRow { room: "A101", rtype:"Condo", status: "Occupied", rent: "6500", tenant: "Boss K." }
                            RoomRow { room: "A102", rtype:"Condo", status: "Vacant", rent: "6200", tenant: "-" }
                            RoomRow { room: "B202", rtype:"Apartment", status: "Occupied", rent: "7100", tenant: "Nicha" }
                            RoomRow { room: "A203", rtype:"Apartment", status: "Vacant", rent: "6500", tenant: "-" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SmallKpi(title: &'static str, value: &'static str, note: &'static str) -> Element {
    rsx! {
        div { class: "panel p-4",
            div { class: "text-xs text-slate-400", "{title}"}
            div { class: "mt-1 text-lg font-semibold text-slate-100", "{value}"}
            div { class: "mt-1 text-xs text-slate-500", "{note}"}
        }
    }
}

#[component]
fn ActivityRow(icon: &'static str, title: &'static str, sub: &'static str) -> Element {
    rsx! {
        li {class: "flex items-start gap-3",
            div { class: "h-9 w-9 rounded-2xl border border-slate-800 bg-slate-950/30 flex items-center justify-center",
                span {"{icon}"}
            }

            div {class: "min-w-0",
                div { class: "font-medium text-slate-100 truncate", "{title}"}
                div { class: "text-xs text-slate-400 truncate", "{sub}" }
            }
        }
    }
}

#[component]
fn RoomRow(
    room: &'static str,
    rtype: &'static str,
    status: &'static str,
    rent: &'static str,
    tenant: &'static str,
) -> Element {
    let pill = match status {
        "Occupied" => "border-emerald-500/25 bg-emerald-500/10 text-emerald-100",
        "Vacant" => "border-slate-700 bg-slate-900/50 text-slate-200",
        "Maintenance" => "border-amber-500/25 bg-amber-500/10 text-amber-100",
        _ => "border-slate-700 bg-slate-900/50 text-slate-200",
    };

    rsx! {
        tr { class: "border-t border-slate-800/80",
            td { class: "py-3 pr-4 font-medium", "{room}" }
            td { class: "py-3 pr-4 text-slate-300", "{rtype}" }
            td { class: "py-3 pr-4",
                span { class: format!("inline-flex items-center rounded-full border px-2 py-1 text-xs {}", pill),
                    "{status}"
                }
            }
            td { class: "py-3 pr-4 text-slate-200", "{rent}" }
            td { class: "py-3 text-slate-300", "{tenant}" }
        }
    }
}
