/// Terminal data model and compile-time CSV loader.
///
/// The CSV is embedded at compile time via `include_str!` so there are
/// no runtime file-system dependencies; the data is baked into the WASM binary
/// just like it was before, but the source-of-truth lives in `assets/terminals.csv`.
///
/// CSV format (header row required):
///   code,series,brand,pliers,conductor,sheath
///
/// Die columns use `|`-separated `AWG:die` pairs, e.g. `24:1.6|22:1.9`.
/// `X` as the die value means crimping impossible → `None`.
/// An empty field means no entries for that barrel.

#[derive(Clone, PartialEq, Debug)]
pub struct Terminal {
    pub code: String,
    pub series: String,
    pub brand: String,
    pub pliers: String,
    pub conductor: Vec<(u8, Option<f32>)>,
    pub sheath: Vec<(u8, Option<f32>)>,
}

/// Colour for a given die number, matching the Engineer Inc. chart legend.
pub fn die_color(die: f32) -> &'static str {
    match (die * 10.0).round() as u32 {
        10 => "#f0a0c8",
        14 => "#f5d08a",
        16 => "#f5e97a",
        19 => "#98e898",
        20 => "#80c8f0",
        23 => "#b090e0",
        _  => "#cccccc",
    }
}

pub fn die_label(die: f32) -> String {
    format!("{:.1}", die)
}

/// Parse `AWG:die|AWG:die` into `Vec<(u8, Option<f32>)>`.
fn parse_dies(s: &str) -> Vec<(u8, Option<f32>)> {
    if s.trim().is_empty() {
        return vec![];
    }
    s.split('|')
        .filter_map(|pair| {
            let mut parts = pair.trim().splitn(2, ':');
            let awg  = parts.next()?.trim().parse::<u8>().ok()?;
            let die_s = parts.next()?.trim();
            let die = if die_s.eq_ignore_ascii_case("x") {
                None
            } else {
                die_s.parse::<f32>().ok().map(Some).unwrap_or(None)
            };
            Some((awg, die))
        })
        .collect()
}

/// Load all terminals from the bundled CSV.
/// Called once at startup; the `include_str!` path is relative to this source file.
pub fn load_terminals() -> Vec<Terminal> {
    let csv = include_str!("../assets/terminals.csv");
    csv.lines()
        .skip(1) // skip header
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            // Split on commas but respect that some fields contain no commas
            // (simple split is fine — none of our fields contain commas).
            let cols: Vec<&str> = line.splitn(6, ',').collect();
            if cols.len() < 6 {
                return None;
            }
            Some(Terminal {
                code:      cols[0].to_string(),
                series:    cols[1].to_string(),
                brand:     cols[2].to_string(),
                pliers:    cols[3].to_string(),
                conductor: parse_dies(cols[4]),
                sheath:    parse_dies(cols[5]),
            })
        })
        .collect()
}
