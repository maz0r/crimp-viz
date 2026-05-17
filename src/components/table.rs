use dioxus::prelude::*;
use crate::data::Terminal;
use crate::components::die_bar::die_pills;

pub fn pliers_class(pliers: &str) -> &'static str {
    match pliers {
        "PA-09" => "pliers-badge pliers-09",
        "PA-20" => "pliers-badge pliers-20",
        _       => "pliers-badge pliers-both",
    }
}

#[component]
pub fn TerminalTable(
    filtered: Vec<(usize, Terminal)>,
    selected: Signal<Option<usize>>,
) -> Element {
    rsx! {
        div { class: "table-wrap",
            table { class: "term-table",
                thead {
                    tr {
                        th { "Code" }
                        th { "Series" }
                        th { "Brand" }
                        th { "Pliers" }
                        th { "Conductor dies" }
                        th { "Sheath dies" }
                    }
                }
                tbody {
                    for (idx, t) in filtered.clone() {
                        {
                            let is_sel     = *selected.read() == Some(idx);
                            let row_cls    = if is_sel { "row row-sel" } else { "row" };
                            let brand_lc   = t.brand.to_lowercase();
                            let pliers_cls = pliers_class(&t.pliers);
                            let cond       = t.conductor.clone();
                            let shth       = t.sheath.clone();
                            rsx! {
                                tr {
                                    class: "{row_cls}",
                                    onclick: move |_| {
                                        let cur = *selected.read();
                                        selected.set(if cur == Some(idx) { None } else { Some(idx) });
                                    },
                                    td { class: "code-cell", "{t.code}" }
                                    td { "{t.series}" }
                                    td { span { class: "brand-badge brand-{brand_lc}", "{t.brand}" } }
                                    td { span { class: "{pliers_cls}", "{t.pliers}" } }
                                    td { { die_pills(cond) } }
                                    td { { die_pills(shth) } }
                                }
                            }
                        }
                    }
                    if filtered.is_empty() {
                        tr {
                            td { colspan: "6", class: "empty-row",
                                "No terminals match your filters."
                            }
                        }
                    }
                }
            }
        }
    }
}
