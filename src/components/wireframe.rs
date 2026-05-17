use dioxus::prelude::*;

const PA09_DIES: &[(f32, &str)] = &[(1.0,"1.0"),(1.4,"1.4"),(1.6,"1.6"),(1.9,"1.9")];
const PA20_DIES: &[(f32, &str)] = &[(1.6,"1.6"),(1.9,"1.9"),(2.0,"2.0"),(2.3,"2.3")];

#[component]
pub fn DieWireframe(
    pliers: String,
    conductor_dies: Vec<f32>,
    sheath_dies: Vec<f32>,
) -> Element {
    let show_09 = pliers == "PA-09" || pliers == "PA-09/PA-20";
    let show_20 = pliers == "PA-20" || pliers == "PA-09/PA-20";
    rsx! {
        div { class: "wireframe-wrap",
            if show_09 {
                p { class: "wf-label", "PA-09" }
                { die_block_svg(PA09_DIES, &conductor_dies, &sheath_dies) }
            }
            if show_20 {
                p { class: "wf-label", "PA-20" }
                { die_block_svg(PA20_DIES, &conductor_dies, &sheath_dies) }
            }
        }
    }
}

fn die_block_svg(
    dies: &[(f32, &str)],
    conductor_dies: &[f32],
    sheath_dies: &[f32],
) -> Element {
    let scale: f32   = 40.0;
    let pad:   f32   = 18.0;
    let gap:   f32   = 10.0;
    let depth: f32   = 32.0;
    let wall:  f32   = 5.0;
    let body_h: f32  = depth + wall + 24.0;
    let label_h: f32 = 18.0;

    let total_w: f32 = pad * 2.0
        + dies.iter().map(|(mm, _)| mm * scale + wall * 2.0).sum::<f32>()
        + gap * (dies.len() as f32 - 1.0);
    let total_h: f32 = pad + body_h + label_h + pad * 0.5;

    // (outer_x, w_px, label, is_cond, is_sheath)
    let mut cavities: Vec<(f32, f32, &str, bool, bool)> = Vec::new();
    let mut cur_x = pad;
    for (mm, label) in dies {
        let w_px   = mm * scale;
        let full_w = w_px + wall * 2.0;
        let is_cond   = conductor_dies.iter().any(|d| (d - mm).abs() < 0.05);
        let is_sheath = sheath_dies.iter().any(|d| (d - mm).abs() < 0.05);
        cavities.push((cur_x, w_px, label, is_cond, is_sheath));
        cur_x += full_w + gap;
    }

    let mut s = String::new();
    s.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {tw:.1} {th:.1}\" width=\"{tw:.1}\" height=\"{th:.1}\">",
        tw = total_w, th = total_h
    ));

    // background
    s.push_str(&format!("<rect width=\"{:.1}\" height=\"{:.1}\" rx=\"6\" fill=\"#1e2230\"/>", total_w, total_h));

    // steel body
    let block_y = pad;
    s.push_str(&format!(
        "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\" rx=\"4\" fill=\"#2a2f42\" stroke=\"#4a5068\" stroke-width=\"1.5\"/>",
        x = pad * 0.5, y = block_y, w = total_w - pad, h = body_h
    ));

    for (out_x_c, w_px_c, label_c, is_cond_c, is_sheath_c) in &cavities {
        let cav_x = out_x_c + wall;
        let cav_y = block_y + wall;
        let cav_w = *w_px_c;
        let out_x = *out_x_c;
        let out_w = w_px_c + wall * 2.0;

        // All colour literals are assigned to variables and concatenated
        // separately — never placed inside format! strings — to avoid the
        // Rust 2021 "unknown prefix" error on `#rrggbb` hex values.
        let fill_col = match (is_cond_c, is_sheath_c) {
            (true,  true)  => "#7a5500",
            (true,  false) => "#0d3d4f",
            (false, true)  => "#4f3d0d",
            _              => "#13161f",
        };
        let stroke_col = match (is_cond_c, is_sheath_c) {
            (true,  true)  => "#f5c84a",
            (true,  false) => "#3dd6b0",
            (false, true)  => "#f5a020",
            _              => "#3a3f55",
        };
        let glow_op = if *is_cond_c || *is_sheath_c { "0.25" } else { "0" };

        // glow rect
        s.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\" rx=\"3\" fill=\"",
            x = out_x - 2.0, y = cav_y - 2.0, w = out_w + 4.0, h = depth + 4.0
        ));
        s.push_str(stroke_col);
        s.push_str(&format!("\" opacity=\"{}\"/>", glow_op));

        // U-channel path (open at top)
        let (x1, y1) = (out_x,         cav_y);
        let (x2, y2) = (out_x,         cav_y + depth);
        let (x3, y3) = (out_x + out_w, cav_y + depth);
        let x4       = out_x + out_w;
        let u = format!(
            "M {x1:.1} {y1:.1} L {x2:.1} {y2:.1} L {x3:.1} {y3:.1} L {x4:.1} {y1:.1} Z",
            x1=x1, y1=y1, x2=x2, y2=y2, x3=x3, y3=y3, x4=x4
        );
        s.push_str("<path d=\"");
        s.push_str(&u);
        s.push_str("\" fill=\"");   s.push_str(fill_col);
        s.push_str("\" stroke=\""); s.push_str(stroke_col);
        s.push_str("\" stroke-width=\"1.5\" stroke-linejoin=\"round\"/>");

        // inner slot
        s.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\" fill=\"",
            x = cav_x, y = cav_y, w = cav_w, h = depth
        ));
        s.push_str(fill_col);
        s.push_str("\" opacity=\"0.6\"/>");

        // indicator dots
        if *is_cond_c {
            let (dcx, dcy) = (cav_x + cav_w * 0.35, cav_y + depth - 6.0);
            s.push_str(&format!("<circle cx=\"{dcx:.1}\" cy=\"{dcy:.1}\" r=\"3\" fill=\"#3dd6b0\"/>", dcx=dcx, dcy=dcy));
        }
        if *is_sheath_c {
            let (dcx, dcy) = (cav_x + cav_w * 0.65, cav_y + depth - 6.0);
            s.push_str(&format!("<circle cx=\"{dcx:.1}\" cy=\"{dcy:.1}\" r=\"3\" fill=\"#f5a020\"/>", dcx=dcx, dcy=dcy));
        }

        // dimension line + label
        let dim_y = block_y + body_h + 6.0;
        let (lx, rx) = (cav_x, cav_x + cav_w);
        let mid      = (lx + rx) / 2.0;

        s.push_str(&format!("<line x1=\"{lx:.1}\" y1=\"{y:.1}\" x2=\"{rx:.1}\" y2=\"{y:.1}\" stroke=\"", lx=lx, rx=rx, y=dim_y));
        s.push_str(stroke_col);
        s.push_str("\" stroke-width=\"0.8\" opacity=\"0.7\"/>");

        for tick_x in [lx, rx] {
            s.push_str(&format!("<line x1=\"{x:.1}\" y1=\"{ya:.1}\" x2=\"{x:.1}\" y2=\"{yb:.1}\" stroke=\"", x=tick_x, ya=dim_y-3.0, yb=dim_y+3.0));
            s.push_str(stroke_col);
            s.push_str("\" stroke-width=\"0.8\" opacity=\"0.7\"/>");
        }

        s.push_str(&format!("<text x=\"{mx:.1}\" y=\"{y:.1}\" text-anchor=\"middle\" font-family=\"monospace\" font-size=\"9\" fill=\"", mx=mid, y=dim_y+14.0));
        s.push_str(stroke_col);
        s.push_str("\">");
        s.push_str(label_c);
        s.push_str("</text>");
    }

    // legend
    let leg_y = total_h - 10.0;
    let leg_x = pad * 0.5 + 4.0;
    s.push_str(&format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3.5\" fill=\"#3dd6b0\"/>", x=leg_x,      y=leg_y));
    s.push_str(&format!("<text x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"sans-serif\" font-size=\"8.5\" fill=\"#8890a8\">conductor</text>", x=leg_x+7.0,  y=leg_y+3.0));
    s.push_str(&format!("<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3.5\" fill=\"#f5a020\"/>", x=leg_x+72.0, y=leg_y));
    s.push_str(&format!("<text x=\"{x:.1}\" y=\"{y:.1}\" font-family=\"sans-serif\" font-size=\"8.5\" fill=\"#8890a8\">sheath</text>",    x=leg_x+79.0, y=leg_y+3.0));

    s.push_str("</svg>");

    rsx! { div { class: "wf-svg", dangerous_inner_html: "{s}" } }
}
