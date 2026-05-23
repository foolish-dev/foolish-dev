use std::fmt::Write as _;
use std::path::PathBuf;

const ROW_LABEL_X: u32 = 280;
const ROW_VALUE_X: u32 = 380;
const ROW_Y_FIRST: u32 = 100;
const ROW_Y_STEP: u32 = 22;

enum Item {
    Primary(&'static str),
    Dim(&'static str),
}

struct Row {
    label: &'static str,
    sep: &'static str,
    items: &'static [Item],
}

const ROWS: &[Row] = &[
    Row {
        label: "os",
        sep: "·",
        items: &[Item::Primary("Arch Linux"), Item::Dim("x86_64")],
    },
    Row {
        label: "wm",
        sep: "·",
        items: &[
            Item::Primary("niri"),
            Item::Dim("scrollable-tiling wayland"),
        ],
    },
    Row {
        label: "shell",
        sep: "·",
        items: &[
            Item::Primary("zsh"),
            Item::Primary("tmux"),
            Item::Primary("neovim"),
        ],
    },
    Row {
        label: "theme",
        sep: "+",
        items: &[Item::Primary("tokyo night"), Item::Dim("pywal")],
    },
    Row {
        label: "agents",
        sep: "·",
        items: &[
            Item::Primary("claude-code"),
            Item::Primary("opencode"),
            Item::Primary("lm-studio"),
            Item::Primary("hexstrike-ai"),
        ],
    },
    Row {
        label: "models",
        sep: "·",
        items: &[Item::Primary("Janus-35B"), Item::Primary("Thanatos-27B")],
    },
];

struct Theme {
    file: &'static str,
    bg_start: &'static str,
    bg_end: &'static str,
    accent: [&'static str; 3],
    mark: [&'static str; 3],
    glow_stddev: &'static str,
    grid: &'static str,
    grid_opacity: &'static str,
    border: &'static str,
    caption: &'static str,
    divider: &'static str,
    at_sign: &'static str,
    host: &'static str,
    mark_mid: &'static str,
    label_fill: &'static str,
    primary: &'static str,
    dim: &'static str,
    sep_fill: &'static str,
    palette: [&'static str; 6],
    palette_note: &'static str,
}

const DARK: Theme = Theme {
    file: "banner.svg",
    bg_start: "#1a1b26",
    bg_end: "#16161e",
    accent: ["#7aa2f7", "#bb9af7", "#7dcfff"],
    mark: ["#7dcfff", "#7aa2f7", "#bb9af7"],
    glow_stddev: "2.5",
    grid: "#414868",
    grid_opacity: "0.3",
    border: "#414868",
    caption: "#565f89",
    divider: "#414868",
    at_sign: "#565f89",
    host: "#7dcfff",
    mark_mid: "#7aa2f7",
    label_fill: "#bb9af7",
    primary: "#c0caf5",
    dim: "#9aa5ce",
    sep_fill: "#565f89",
    palette: [
        "#f7768e", "#e0af68", "#9ece6a", "#7dcfff", "#7aa2f7", "#bb9af7",
    ],
    palette_note: "TN palette dots",
};

const LIGHT: Theme = Theme {
    file: "banner-light.svg",
    bg_start: "#e1e2e7",
    bg_end: "#d5d6db",
    accent: ["#2e7de9", "#9854f1", "#007197"],
    mark: ["#007197", "#2e7de9", "#9854f1"],
    glow_stddev: "1.6",
    grid: "#a8aecb",
    grid_opacity: "0.45",
    border: "#a8aecb",
    caption: "#848cb5",
    divider: "#a8aecb",
    at_sign: "#848cb5",
    host: "#007197",
    mark_mid: "#2e7de9",
    label_fill: "#9854f1",
    primary: "#3760bf",
    dim: "#6172b0",
    sep_fill: "#848cb5",
    palette: [
        "#f52a65", "#b15c00", "#587539", "#007197", "#2e7de9", "#9854f1",
    ],
    palette_note: "TN-day palette dots",
};

fn write_row(t: &Theme, row: &Row, y: u32, w: &mut String) {
    writeln!(w, "    <text y=\"{y}\">").unwrap();
    writeln!(
        w,
        "      <tspan x=\"{ROW_LABEL_X}\" fill=\"{}\" font-weight=\"600\">{}</tspan>",
        t.label_fill, row.label
    )
    .unwrap();
    for (i, item) in row.items.iter().enumerate() {
        let (text, fill) = match item {
            Item::Primary(s) => (*s, t.primary),
            Item::Dim(s) => (*s, t.dim),
        };
        if i == 0 {
            writeln!(
                w,
                "      <tspan x=\"{ROW_VALUE_X}\" fill=\"{fill}\">{text}</tspan>"
            )
            .unwrap();
        } else {
            writeln!(
                w,
                "      <tspan dx=\"8\" fill=\"{}\">{}</tspan>",
                t.sep_fill, row.sep
            )
            .unwrap();
            writeln!(w, "      <tspan dx=\"6\" fill=\"{fill}\">{text}</tspan>").unwrap();
        }
    }
    writeln!(w, "    </text>").unwrap();
}

fn render(t: &Theme) -> String {
    let mut s = String::with_capacity(4096);
    let w = &mut s;

    writeln!(
        w,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 900 240" role="img" aria-label="foolish-dev — neofetch banner">"#
    )
    .unwrap();
    writeln!(w, "  <defs>").unwrap();
    writeln!(
        w,
        r#"    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">"#
    )
    .unwrap();
    writeln!(
        w,
        r#"      <stop offset="0%" stop-color="{}"/>"#,
        t.bg_start
    )
    .unwrap();
    writeln!(
        w,
        r#"      <stop offset="100%" stop-color="{}"/>"#,
        t.bg_end
    )
    .unwrap();
    writeln!(w, "    </linearGradient>").unwrap();
    writeln!(
        w,
        r#"    <linearGradient id="accent" x1="0" y1="0" x2="1" y2="0">"#
    )
    .unwrap();
    writeln!(
        w,
        r#"      <stop offset="0%" stop-color="{}"/>"#,
        t.accent[0]
    )
    .unwrap();
    writeln!(
        w,
        r#"      <stop offset="50%" stop-color="{}"/>"#,
        t.accent[1]
    )
    .unwrap();
    writeln!(
        w,
        r#"      <stop offset="100%" stop-color="{}"/>"#,
        t.accent[2]
    )
    .unwrap();
    writeln!(w, "    </linearGradient>").unwrap();
    writeln!(
        w,
        r#"    <linearGradient id="mark" x1="0" y1="0" x2="0" y2="1">"#
    )
    .unwrap();
    writeln!(w, r#"      <stop offset="0%" stop-color="{}"/>"#, t.mark[0]).unwrap();
    writeln!(
        w,
        r#"      <stop offset="50%" stop-color="{}"/>"#,
        t.mark[1]
    )
    .unwrap();
    writeln!(
        w,
        r#"      <stop offset="100%" stop-color="{}"/>"#,
        t.mark[2]
    )
    .unwrap();
    writeln!(w, "    </linearGradient>").unwrap();
    writeln!(
        w,
        r#"    <filter id="glow" x="-30%" y="-30%" width="160%" height="160%">"#
    )
    .unwrap();
    writeln!(
        w,
        r#"      <feGaussianBlur stdDeviation="{}" result="b"/>"#,
        t.glow_stddev
    )
    .unwrap();
    writeln!(
        w,
        r#"      <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>"#
    )
    .unwrap();
    writeln!(w, "    </filter>").unwrap();
    writeln!(
        w,
        r#"    <pattern id="grid" width="24" height="24" patternUnits="userSpaceOnUse">"#
    )
    .unwrap();
    writeln!(
        w,
        r#"      <path d="M 24 0 L 0 0 0 24" fill="none" stroke="{}" stroke-width="0.4" opacity="{}"/>"#,
        t.grid, t.grid_opacity
    )
    .unwrap();
    writeln!(w, "    </pattern>").unwrap();
    writeln!(w, "  </defs>").unwrap();
    writeln!(w).unwrap();

    writeln!(
        w,
        r#"  <rect x="0" y="0" width="900" height="240" rx="14" fill="url(#bg)"/>"#
    )
    .unwrap();
    writeln!(
        w,
        r#"  <rect x="0" y="0" width="900" height="240" rx="14" fill="url(#grid)"/>"#
    )
    .unwrap();
    writeln!(
        w,
        r#"  <rect x="0.5" y="0.5" width="899" height="239" rx="14" fill="none" stroke="{}" stroke-width="1"/>"#,
        t.border
    )
    .unwrap();
    writeln!(w).unwrap();

    writeln!(w, "  <!-- arch-mountain mark -->").unwrap();
    writeln!(
        w,
        r#"  <g transform="translate(130,118)" filter="url(#glow)">"#
    )
    .unwrap();
    writeln!(
        w,
        r#"    <polygon points="0,-70 70,70 -70,70" fill="none" stroke="url(#mark)" stroke-width="2.5" stroke-linejoin="round"/>"#
    )
    .unwrap();
    writeln!(
        w,
        r#"    <polygon points="0,-42 52,56 -52,56" fill="none" stroke="{}" stroke-width="1.4" stroke-linejoin="round" opacity="0.55"/>"#,
        t.mark_mid
    )
    .unwrap();
    writeln!(
        w,
        r#"    <polygon points="0,-14 28,42 -28,42" fill="url(#mark)" opacity="0.9"/>"#
    )
    .unwrap();
    writeln!(w, "  </g>").unwrap();
    writeln!(
        w,
        r#"  <text x="130" y="222" font-family="JetBrains Mono, ui-monospace, monospace" font-size="11" fill="{}" text-anchor="middle">/* arch · niri */</text>"#,
        t.caption
    )
    .unwrap();
    writeln!(w).unwrap();

    writeln!(
        w,
        r#"  <line x1="248" y1="48" x2="248" y2="192" stroke="{}" stroke-width="1"/>"#,
        t.divider
    )
    .unwrap();
    writeln!(w).unwrap();

    writeln!(w, "  <!-- header -->").unwrap();
    writeln!(
        w,
        r#"  <text x="280" y="58" font-family="JetBrains Mono, ui-monospace, monospace" font-size="22" font-weight="700" filter="url(#glow)">"#
    )
    .unwrap();
    writeln!(w, r#"    <tspan fill="url(#accent)">foolish-dev</tspan>"#).unwrap();
    writeln!(
        w,
        r#"    <tspan dx="6" fill="{}" font-weight="500">@</tspan>"#,
        t.at_sign
    )
    .unwrap();
    writeln!(w, r#"    <tspan dx="4" fill="{}">arch</tspan>"#, t.host).unwrap();
    writeln!(w, "  </text>").unwrap();
    writeln!(
        w,
        r#"  <line x1="280" y1="72" x2="860" y2="72" stroke="{}" stroke-width="1"/>"#,
        t.divider
    )
    .unwrap();
    writeln!(w).unwrap();

    writeln!(w, "  <!-- info rows -->").unwrap();
    writeln!(
        w,
        r#"  <g font-family="JetBrains Mono, ui-monospace, monospace" font-size="14">"#
    )
    .unwrap();
    for (i, row) in ROWS.iter().enumerate() {
        let y = ROW_Y_FIRST + (i as u32) * ROW_Y_STEP;
        write_row(t, row, y, w);
    }
    writeln!(w, "  </g>").unwrap();
    writeln!(w).unwrap();

    writeln!(w, "  <!-- {} -->", t.palette_note).unwrap();
    writeln!(w, r#"  <g transform="translate(778,218)">"#).unwrap();
    for (i, color) in t.palette.iter().enumerate() {
        let cx = (i as u32) * 13;
        writeln!(w, r#"    <circle cx="{cx}" cy="0" r="4" fill="{color}"/>"#).unwrap();
    }
    writeln!(w, "  </g>").unwrap();
    writeln!(w, "</svg>").unwrap();
    s
}

fn main() -> std::io::Result<()> {
    let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("assets");
    for theme in [&DARK, &LIGHT] {
        let path = assets.join(theme.file);
        std::fs::write(&path, render(theme))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}
