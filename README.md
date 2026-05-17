# Crimp Viz — PA-09 / PA-20 Die Reference

A Dioxus web app for visualising crimping die sizes from the Engineer Inc.
PA-09 and PA-20 application chart, covering JST, Molex, AMP, JAE, and HRS terminals.

## Features

- 60+ terminals across 5 brands, with conductor and sheath die numbers
- Colour-coded die pills matching the original chart legend
- Filter by brand, by pliers model (PA-09 / PA-20 / both), or free-text search
- Click any row for a detail panel with die bars and a to-scale SVG die block

## Prerequisites

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Add the WASM target
rustup target add wasm32-unknown-unknown

# 3. Install the Dioxus CLI
cargo install dioxus-cli
```

## Development

```bash
dx serve
# hot-reloading dev server at http://localhost:8080
```

## Release build + serve

```bash
dx build --release
# output: target/dx/crimp-viz/release/web/public/
```

> **Important:** always serve over HTTP, never open `index.html` directly
> from the filesystem.  Browsers block ES-module / WASM imports on `file://`
> due to CORS policy.

```bash
# Option A — Python (no install needed)
cd target/dx/crimp-viz/release/web/public
python3 -m http.server 8080
# open http://localhost:8080

# Option B — dx itself
dx serve --release

# Option C — npx serve
npx serve target/dx/crimp-viz/release/web/public
```

## Deployment

Copy `target/dx/crimp-viz/release/web/public/` to any static host
(Netlify, GitHub Pages, nginx, Caddy, S3 + CloudFront, etc.).
No server-side logic is required.

### GitHub Pages

```bash
# build with the correct base path (must match your repo name)
dx build --release --base-path crimp-viz
# push public/ to gh-pages branch
```

## Project structure

```
assets/
  index.html        Dioxus HTML mount point
  style.css         All application styles
  terminals.csv     Terminal data — edit here to add/remove terminals
src/
  main.rs           Entry point + App component
  data.rs           Terminal model + CSV parser (embedded at compile time)
  components/
    controls.rs     Header, legend, search/filter bar
    table.rs        Terminal data table
    die_bar.rs      Die pills (table cells) and die bars (detail panel)
    detail.rs       Detail panel
    wireframe.rs    To-scale SVG die block visualisation
```

## Adding terminals

Edit `assets/terminals.csv`:

```
code,series,brand,pliers,conductor,sheath
SPH-002T-P0.5L,PH/PAD/KRW,JST,PA-09/PA-20,24:1.6|22:1.6,24:1.9|22:1.9
```

- **pliers**: `PA-09`, `PA-20`, or `PA-09/PA-20`
- **conductor / sheath**: `|`-separated `AWG:die` pairs, e.g. `24:1.6|22:1.9`
- Use `X` as the die value for "crimping impossible"
- Leave a field empty if there are no entries for that barrel

## Colour legend

| Die No. | Colour |
|---------|--------|
| 1.0     | Pink   |
| 1.4     | Amber  |
| 1.6     | Yellow |
| 1.9     | Green  |
| 2.0     | Blue   |
| 2.3     | Purple |
| X       | Red (impossible) |

## Data source

Engineer Inc. PA-09 / PA-20 Application Chart, revision 2027-05-04.
