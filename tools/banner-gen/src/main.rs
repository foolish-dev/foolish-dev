use std::fmt::Write as _;
use std::path::PathBuf;

// A self-contained Tokyo Night terminal window: titlebar + traffic lights, an
// arch-mountain mark in the left gutter, and a neofetch readout on the right.
// The card paints its own background so it reads on both light and dark GitHub.
//
// Row values are emitted into the SVG verbatim (no XML escaping). Keep every
// key/value free of XML metacharacters (& < > ") and of `--`. Static data only.
struct Row {
    key: &'static str,
    val: &'static str,
}

const ROWS: &[Row] = &[
    Row {
        key: "os",
        val: "arch linux · x86_64",
    },
    Row {
        key: "wm",
        val: "niri · scrollable-tiling wayland",
    },
    Row {
        key: "shell",
        val: "zsh",
    },
    Row {
        key: "editor",
        val: "neovim",
    },
    Row {
        key: "theme",
        val: "tokyo night + pywal + noctalia",
    },
    Row {
        key: "agents",
        val: "teleia · claude-code · hexstrike-ai",
    },
    Row {
        key: "models",
        val: "Janus-35B · Thanatos-27B",
    },
];

// Tokyo Night accent dots, bottom-right — the maker's mark.
const PALETTE: [&str; 6] = [
    "#f7768e", "#e0af68", "#9ece6a", "#7dcfff", "#7aa2f7", "#bb9af7",
];

const ROW_KEY_X: u32 = 324;
const ROW_VAL_X: u32 = 430;
const ROW_Y0: u32 = 96;
const ROW_STEP: u32 = 22;

fn render() -> String {
    let mut s = String::with_capacity(4096);
    let w = &mut s;

    writeln!(
        w,
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 900 240" role="img" aria-label="foolish-dev — a themed terminal session on arch + niri + tokyo night">"##
    )
    .unwrap();

    // defs: mark gradient, rounded-card clip, and a soft glow.
    writeln!(w, "  <defs>").unwrap();
    writeln!(
        w,
        r##"    <linearGradient id="mark" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stop-color="#7aa2f7"/><stop offset="100%" stop-color="#bb9af7"/></linearGradient>"##
    )
    .unwrap();
    writeln!(
        w,
        r##"    <clipPath id="card"><rect x="0" y="0" width="900" height="240" rx="12"/></clipPath>"##
    )
    .unwrap();
    writeln!(
        w,
        r##"    <filter id="glow" x="-20%" y="-20%" width="140%" height="140%"><feGaussianBlur stdDeviation="1.4" result="b"/><feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge></filter>"##
    )
    .unwrap();
    writeln!(w, "  </defs>").unwrap();

    // window body, titlebar, and terminal canvas — clipped to rounded corners.
    writeln!(
        w,
        r##"  <rect x="0" y="0" width="900" height="240" rx="12" fill="#24283b"/>"##
    )
    .unwrap();
    writeln!(w, r##"  <g clip-path="url(#card)">"##).unwrap();
    writeln!(
        w,
        r##"    <rect x="0" y="0" width="900" height="30" fill="#2f334d"/>"##
    )
    .unwrap();
    writeln!(
        w,
        r##"    <rect x="0" y="30" width="900" height="210" fill="#1a1b26"/>"##
    )
    .unwrap();
    writeln!(
        w,
        r##"    <line x1="0" y1="30" x2="900" y2="30" stroke="#1b1e2e" stroke-width="1"/>"##
    )
    .unwrap();

    // everything textual shares the mono stack.
    writeln!(
        w,
        r##"    <g font-family="ui-monospace, 'JetBrains Mono', 'Fira Code', monospace">"##
    )
    .unwrap();

    // titlebar: traffic lights + centered tab label.
    for (i, fill) in ["#f7768e", "#e0af68", "#9ece6a"].iter().enumerate() {
        let cx = 18 + i as u32 * 20;
        writeln!(
            w,
            r##"      <circle cx="{cx}" cy="15" r="5" fill="{fill}"/>"##
        )
        .unwrap();
    }
    writeln!(
        w,
        r##"      <text x="450" y="19.5" text-anchor="middle" font-size="12" font-weight="500" fill="#a9b1d6">foolish@arch: ~/.dotfiles</text>"##
    )
    .unwrap();

    // left gutter: arch-mountain mark (two nested outlines) + caption.
    writeln!(w, r##"      <g filter="url(#glow)" fill="none" stroke="url(#mark)" stroke-width="2" stroke-linejoin="round">"##).unwrap();
    writeln!(w, r##"        <polygon points="150,66 98,176 202,176"/>"##).unwrap();
    writeln!(
        w,
        r##"        <polygon points="150,104 126,176 174,176"/>"##
    )
    .unwrap();
    writeln!(w, "      </g>").unwrap();
    writeln!(
        w,
        r##"      <text x="150" y="202" text-anchor="middle" font-size="11" fill="#565f89">/* arch · niri */</text>"##
    )
    .unwrap();
    writeln!(
        w,
        r##"      <line x1="300" y1="46" x2="300" y2="206" stroke="#414868" stroke-width="1"/>"##
    )
    .unwrap();

    // right content: the live prompt, glowing to match the mark.
    writeln!(
        w,
        r##"      <text x="{ROW_KEY_X}" y="66" font-size="14" xml:space="preserve" filter="url(#glow)"><tspan fill="#9ece6a" font-weight="600">foolish@arch</tspan><tspan fill="#7aa2f7">:~</tspan><tspan fill="#bb9af7">$</tspan><tspan fill="#c0caf5"> neofetch</tspan></text>"##
    )
    .unwrap();

    // neofetch rows.
    for (i, row) in ROWS.iter().enumerate() {
        let y = ROW_Y0 + i as u32 * ROW_STEP;
        writeln!(
            w,
            r##"      <text x="{ROW_KEY_X}" y="{y}" font-size="13" font-weight="600" fill="#7dcfff">{}</text>"##,
            row.key
        )
        .unwrap();
        writeln!(
            w,
            r##"      <text x="{ROW_VAL_X}" y="{y}" font-size="13" fill="#a9b1d6">{}</text>"##,
            row.val
        )
        .unwrap();
    }

    // blinking block cursor after the last readout line.
    let cursor_y = ROW_Y0 + (ROWS.len() as u32 - 1) * ROW_STEP;
    writeln!(
        w,
        r##"      <rect x="628" y="{}" width="9" height="14" fill="#c0caf5"><animate attributeName="opacity" values="1;1;0;0" dur="1.1s" repeatCount="indefinite"/></rect>"##,
        cursor_y - 11
    )
    .unwrap();

    writeln!(w, "    </g>").unwrap();

    // palette dots, bottom-right.
    for (i, color) in PALETTE.iter().enumerate() {
        let cx = 792 + i as u32 * 14;
        writeln!(
            w,
            r##"    <circle cx="{cx}" cy="222" r="4" fill="{color}"/>"##
        )
        .unwrap();
    }

    writeln!(w, "  </g>").unwrap();
    // border on top so it stays crisp over the fills.
    writeln!(
        w,
        r##"  <rect x="0.5" y="0.5" width="899" height="239" rx="12" fill="none" stroke="#414868" stroke-width="1"/>"##
    )
    .unwrap();
    writeln!(w, "</svg>").unwrap();

    s
}

fn main() -> std::io::Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("assets")
        .join("banner-light.svg");
    std::fs::write(&path, render())?;
    println!("wrote {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // The committed SVG is a generated artifact; guard against it drifting from
    // the generator. If this fails, run `cargo run` in tools/banner-gen.
    #[test]
    fn committed_asset_matches_render() {
        assert_eq!(
            render(),
            include_str!("../../../assets/banner-light.svg"),
            "assets/banner-light.svg is stale — run `cargo run` in tools/banner-gen"
        );
    }

    #[test]
    fn rows_have_no_xml_metacharacters() {
        for row in ROWS {
            for text in [row.key, row.val] {
                assert!(
                    !text.contains(['&', '<', '>', '"']) && !text.contains("--"),
                    "row text must stay XML-safe: {text:?}"
                );
            }
        }
    }
}
