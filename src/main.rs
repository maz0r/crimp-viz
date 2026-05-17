mod components;
mod data;

use dioxus::prelude::*;
use components::{
    controls::{Controls, Header},
    detail::DetailPanel,
    table::TerminalTable,
};
use data::load_terminals;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let terminals = load_terminals();

    // Collect unique brands in source order
    let brands: Vec<String> = {
        let mut seen = std::collections::HashSet::new();
        terminals
            .iter()
            .filter_map(|t| {
                if seen.insert(t.brand.clone()) { Some(t.brand.clone()) } else { None }
            })
            .collect()
    };

    let mut search        = use_signal(|| String::new());
    let mut filter_brand  = use_signal(|| "All".to_string());
    let mut filter_pliers = use_signal(|| "All".to_string());
    let mut selected      = use_signal(|| Option::<usize>::None);

    let filtered: Vec<(usize, data::Terminal)> = terminals
        .into_iter()
        .enumerate()
        .filter(|(_, t)| {
            let q         = search.read().to_lowercase();
            let brand_ok  = *filter_brand.read() == "All" || t.brand == *filter_brand.read();
            let pliers_ok = match filter_pliers.read().as_str() {
                "PA-09" => t.pliers == "PA-09" || t.pliers == "PA-09/PA-20",
                "PA-20" => t.pliers == "PA-20" || t.pliers == "PA-09/PA-20",
                _       => true,
            };
            let search_ok = q.is_empty()
                || t.code.to_lowercase().contains(&q)
                || t.series.to_lowercase().contains(&q);
            brand_ok && pliers_ok && search_ok
        })
        .collect();

    let sel_terminal = selected.read().and_then(|sel_idx| {
        filtered.iter().find(|(idx, _)| *idx == sel_idx).map(|(_, t)| t.clone())
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/style.css") }
        div { class: "app",
            Header {}
            Controls {
                search,
                filter_brand,
                filter_pliers,
                brands,
                result_count: filtered.len(),
            }
            div { class: "main",
                TerminalTable { filtered: filtered.clone(), selected }
                if let Some(t) = sel_terminal {
                    DetailPanel {
                        terminal: t,
                        on_close: move |_| selected.set(None),
                    }
                }
            }
        }
    }
}
