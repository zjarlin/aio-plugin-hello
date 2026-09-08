#![forbid(unsafe_code)]

use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use az_ui_components::button::Button;
use dill::CatalogBuilder;
use dioxus::prelude::*;

#[derive(Debug)]
struct PagesPlugin;

impl ApplicationPlugin for PagesPlugin {
    fn pages(&self) -> Vec<ApplicationPage> {
        vec![ApplicationPage {
            id: "hello",
            label: "Hello",
            icon: Some("panels-top-left"),
            scene: ApplicationScene {
                id: "workspace",
                label: "工作区",
            },
            required_permission: None,
            render: PluginPage,
        }]
    }
}

pub fn register(builder: &mut CatalogBuilder) {
    builder
        .add_value(PagesPlugin)
        .bind::<dyn ApplicationPlugin, PagesPlugin>();
}

#[allow(non_snake_case)]
fn PluginPage() -> Element {
    let mut count = use_signal(|| 0_u64);
    let value = count();
    rsx! {
        section {
            h2 { "Hello" }
            p { "计数：{value}" }
            Button {
                aria_label: "计数加一",
                onclick: move |_| count += 1,
                "加一"
            }
        }
    }
}
