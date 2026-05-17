use dioxus::prelude::*;
use crate::data::{die_color, die_label};

/// Compact die pills used inside table cells.
pub fn die_pills(entries: Vec<(u8, Option<f32>)>) -> Element {
    rsx! {
        div { class: "dies-cell",
            for (awg, die) in entries {
                {
                    let bg    = die.map(die_color).unwrap_or("#ff6b6b");
                    let label = die.map(die_label).unwrap_or("X".into());
                    rsx! {
                        span { class: "die-entry",
                            span { class: "awg-tag", "#{awg}" }
                            span { class: "die-pill", style: "background: {bg}", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

/// Tall coloured bars used in the detail panel.
#[component]
pub fn DieBar(entries: Vec<(u8, Option<f32>)>) -> Element {
    rsx! {
        div { class: "die-bar",
            for (awg, die) in entries {
                {
                    let bg    = die.map(die_color).unwrap_or("#ff6b6b");
                    let label = die.map(die_label).unwrap_or("X".into());
                    rsx! {
                        div { class: "die-bar-item", style: "background: {bg}",
                            span { class: "die-bar-awg", "AWG {awg}" }
                            span { class: "die-bar-num", "Die {label}" }
                        }
                    }
                }
            }
        }
    }
}
