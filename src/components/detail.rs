use dioxus::prelude::*;
use crate::data::Terminal;
use crate::components::{
    die_bar::DieBar,
    table::pliers_class,
    wireframe::DieWireframe,
};

#[component]
pub fn DetailPanel(terminal: Terminal, on_close: EventHandler<()>) -> Element {
    let brand_lc   = terminal.brand.to_lowercase();
    let pliers_cls = pliers_class(&terminal.pliers);
    let cond_dies: Vec<f32> = terminal.conductor.iter().filter_map(|(_, d)| *d).collect();
    let shth_dies: Vec<f32> = terminal.sheath.iter().filter_map(|(_, d)| *d).collect();

    rsx! {
        div { class: "detail-panel",
            div { class: "detail-header",
                h2 { "{terminal.code}" }
                button {
                    class: "close-btn",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }
            div { class: "detail-body",
                div { class: "detail-row",
                    span { class: "detail-label", "Series" }
                    span { "{terminal.series}" }
                }
                div { class: "detail-row",
                    span { class: "detail-label", "Brand" }
                    span { class: "brand-badge brand-{brand_lc}", "{terminal.brand}" }
                }
                div { class: "detail-row",
                    span { class: "detail-label", "Pliers" }
                    span { class: "{pliers_cls}", "{terminal.pliers}" }
                }

                h3 { class: "detail-section-title", "Conductor (AWG → Die No.)" }
                if terminal.conductor.is_empty() {
                    p { class: "none-note", "No conductor data" }
                } else {
                    DieBar { entries: terminal.conductor.clone() }
                }

                h3 { class: "detail-section-title", "Sheath (AWG → Die No.)" }
                if terminal.sheath.is_empty() {
                    p { class: "none-note", "No sheath data" }
                } else {
                    DieBar { entries: terminal.sheath.clone() }
                }

                h3 { class: "detail-section-title", "Die Block Visualisation" }
                DieWireframe {
                    pliers: terminal.pliers.clone(),
                    conductor_dies: cond_dies,
                    sheath_dies: shth_dies,
                }
            }
        }
    }
}
