use dioxus::prelude::*;

const LEGEND_DIES: &[(f32, &str)] = &[
    (1.0, "1.0"), (1.4, "1.4"), (1.6, "1.6"),
    (1.9, "1.9"), (2.0, "2.0"), (2.3, "2.3"),
];

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "app-header",
            div { class: "header-inner",
                div { class: "logo",
                    span { class: "logo-icon", "⚙" }
                    div {
                        h1 { "CRIMP VIZ" }
                        p { class: "subtitle", "PA-09 / PA-20 Die Reference" }
                    }
                }
                div { class: "legend",
                    for (die, label) in LEGEND_DIES {
                        {
                            let bg = crate::data::die_color(*die);
                            rsx! {
                                span { class: "die-pill", style: "background: {bg}", "{label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Controls(
    search: Signal<String>,
    filter_brand: Signal<String>,
    filter_pliers: Signal<String>,
    brands: Vec<String>,
    result_count: usize,
) -> Element {
    let count_label = if result_count == 1 { "terminal" } else { "terminals" };
    rsx! {
        div { class: "controls",
            div { class: "search-wrap",
                span { class: "search-icon", "🔍" }
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: "Search code or series…",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }
            }
            div { class: "filter-group",
                label { "Brand" }
                select {
                    class: "filter-select",
                    onchange: move |e| filter_brand.set(e.value()),
                    option { value: "All", "All brands" }
                    for b in &brands {
                        option { value: "{b}", "{b}" }
                    }
                }
            }
            div { class: "filter-group",
                label { "Pliers" }
                select {
                    class: "filter-select",
                    onchange: move |e| filter_pliers.set(e.value()),
                    option { value: "All",   "PA-09 & PA-20" }
                    option { value: "PA-09", "PA-09 only"    }
                    option { value: "PA-20", "PA-20 only"    }
                }
            }
            div { class: "result-count", "{result_count} {count_label}" }
        }
    }
}
