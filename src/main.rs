// import library

use dioxus::html::g::class;
use dioxus::prelude::*;

// import module components

mod components;

// use modules
use crate::components::sidebar::*;
use crate::components::stat_card::*;
use crate::components::TopBar;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
} // close main

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}
    }
} // close app element

#[component]
pub fn Hero() -> Element {
    rsx! {
        // แนะนำให้ใช้โครงสร้าง Flexbox หรือ Grid เพื่อจัดตำแหน่งร่วมกับ Sidebar
        div { class: "flex h-screen",
            // 1. เรียกใช้ Sidebar
            SideBar {
                collapsed: use_signal(|| false),
                active: use_signal(|| NavKey::DashBoard)
            }

            // 2. ส่วนเนื้อหาขวาที่มี TopBar
            main { class: "flex-1 flex flex-col min-w-0",

                // --- เรียกใช้ TopBar ตรงนี้ ---
                TopBar { title: "Dashboard" }

                // ส่วนเนื้อหาด้านล่าง TopBar
                div { class: "p-4 overflow-y-auto",
                    "เนื้อหาหน้า Dashboard ของคุณ..."

                            StatCard {
                                label: "Total Income",
                                value: "฿45,000",
                                hint: "+12% from last month",
                                icon: "💰"
                            }

                            // แบบระบุ Variant เป็น Sky
                            StatCard {
                                label: "Occupied Rooms",
                                value: "18/20",
                                hint: "2 rooms vacant",
                                icon: "🏠",
                                variant: StatCardVariant::Sky
                            }
                }
            }
        }
    }
} // close Hero function
