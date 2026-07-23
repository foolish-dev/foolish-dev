// Port of the Python steps in .github/workflows/sync-hf-profile.yml.
//
//   hf-sync build   stage README.md for the Hugging Face profile Space into
//                   .hf-out/README.md (GitHub theme-specific image variants
//                   flattened, relative asset refs absolutised, live Space
//                   frontmatter preserved).
//   hf-sync push    commit .hf-out/README.md to the FoolDev/FoolDev Space.
//
// Split into two subcommands so the workflow can slot its dry-run artifact
// upload between them, exactly as the Python version did.

use base64::Engine as _;
use regex::{Captures, Regex};
use std::time::Duration;
use std::{env, fs, process};

const SPACE_REPO: &str = "FoolDev/FoolDev";
const FRONTMATTER_URL: &str = "https://huggingface.co/spaces/FoolDev/FoolDev/raw/main/README.md";
const OUT_PATH: &str = ".hf-out/README.md";

// Used when the live Space README can't be fetched, so a transient failure
// doesn't drop the Space's own metadata (title, sdk, pinned, ...).
const FALLBACK_FRONTMATTER: &str = "---\n\
    title: FoolDev\n\
    emoji: ⚡\n\
    colorFrom: blue\n\
    colorTo: purple\n\
    sdk: static\n\
    pinned: true\n\
    short_description: agentic offsec · Janus 35B + Thanatos 27B · arch+niri\n\
    ---\n";

/// Flatten GitHub-only markup and absolutise relative asset refs so the README
/// renders on Hugging Face, which has no `#gh-{dark,light}-mode-only` support
/// and can't resolve repo-relative image paths.
fn transform_body(readme: &str, base: &str) -> String {
    // Keep the dark variant (canonical); drop the light line and strip the
    // dark fragment so the remaining URL stays clean.
    let light_line = Regex::new(r"(?m)^.*#gh-light-mode-only.*\n?").unwrap();
    let mut body = light_line.replace_all(readme, "").into_owned();
    body = body.replace("#gh-dark-mode-only", "");

    // `src=`/`srcset=` and `![](assets/...)` refs that are repo-relative get
    // the raw.githubusercontent base prepended. A literal `assets/` right after
    // the delimiter is already relative, so no absolute-URL guard is needed.
    let attr = Regex::new(r#"((?:src|srcset)=")(assets/[^"]+)(")"#).unwrap();
    body = attr
        .replace_all(&body, |c: &Captures| {
            format!("{}{}{}{}", &c[1], base, &c[2], &c[3])
        })
        .into_owned();
    let md_img = Regex::new(r"(!\[[^\]]*\]\()(assets/[^)]+)(\))").unwrap();
    body = md_img
        .replace_all(&body, |c: &Captures| {
            format!("{}{}{}{}", &c[1], base, &c[2], &c[3])
        })
        .into_owned();

    // Drop any frontmatter already in the GitHub README; the Space's own is
    // spliced back on in `build`.
    strip_leading_frontmatter(&body)
}

/// Return `text` with a leading `---\n ... \n---\n` block removed, else unchanged.
fn strip_leading_frontmatter(text: &str) -> String {
    if let Some(rest) = text.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            return rest[end + "\n---\n".len()..]
                .trim_start_matches('\n')
                .to_string();
        }
    }
    text.to_string()
}

/// The leading `---\n ... \n---\n` block of `text`, including both fences, or "".
fn extract_leading_frontmatter(text: &str) -> String {
    if let Some(rest) = text.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            return text[.."---\n".len() + end + "\n---\n".len()].to_string();
        }
    }
    String::new()
}

fn fetch_frontmatter(token: &str) -> String {
    let resp = ureq::get(FRONTMATTER_URL)
        .set("Authorization", &format!("Bearer {token}"))
        .timeout(Duration::from_secs(15))
        .call();
    match resp {
        Ok(r) => match r.into_string() {
            Ok(cur) => extract_leading_frontmatter(&cur),
            Err(e) => {
                eprintln!("warning: could not read Space README body: {e}");
                String::new()
            }
        },
        Err(e) => {
            eprintln!("warning: could not fetch existing Space frontmatter: {e}");
            String::new()
        }
    }
}

fn cmd_build() {
    let repo =
        env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "foolish-dev/foolish-dev".to_string());
    let base = format!("https://raw.githubusercontent.com/{repo}/main/");

    let readme = fs::read_to_string("README.md").expect("read README.md");
    let body = transform_body(&readme, &base);

    let frontmatter = match env::var("HF_TOKEN") {
        Ok(t) if !t.is_empty() => fetch_frontmatter(&t),
        _ => {
            eprintln!("warning: HF_TOKEN unset; using fallback frontmatter");
            String::new()
        }
    };
    let frontmatter = if frontmatter.is_empty() {
        FALLBACK_FRONTMATTER.to_string()
    } else {
        frontmatter
    };

    let out = format!("{}\n{}", frontmatter, body.trim_start_matches('\n'));
    fs::create_dir_all(".hf-out").expect("create .hf-out");
    fs::write(OUT_PATH, &out).expect("write staged README");
    println!("hf README: {} bytes (frontmatter preserved)", out.len());
}

fn cmd_push() {
    let token = env::var("HF_TOKEN").expect("HF_TOKEN required for push");
    let repo = env::var("GITHUB_REPOSITORY").unwrap_or_default();
    let sha = env::var("GITHUB_SHA").unwrap_or_default();
    let short: String = sha.chars().take(7).collect();

    let content = fs::read(OUT_PATH).expect("read staged README");
    let b64 = base64::engine::general_purpose::STANDARD.encode(&content);

    // Hugging Face commit API: newline-delimited JSON, a header line then one
    // line per file. A small text file is committed inline as base64.
    let header = serde_json::json!({
        "key": "header",
        "value": { "summary": format!("Sync README from {repo}@{short}"), "description": "" }
    });
    let file = serde_json::json!({
        "key": "file",
        "value": { "path": "README.md", "content": b64, "encoding": "base64" }
    });
    let payload = format!("{header}\n{file}\n");

    let url = format!("https://huggingface.co/api/spaces/{SPACE_REPO}/commit/main");
    match ureq::post(&url)
        .set("Authorization", &format!("Bearer {token}"))
        .set("Content-Type", "application/x-ndjson")
        .send_string(&payload)
    {
        Ok(_) => println!("Uploaded README.md to space {SPACE_REPO}"),
        Err(ureq::Error::Status(code, r)) => {
            let detail = r.into_string().unwrap_or_default();
            eprintln!("HF commit failed: HTTP {code}\n{detail}");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("HF commit request error: {e}");
            process::exit(1);
        }
    }
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("build") => cmd_build(),
        Some("push") => cmd_push(),
        other => {
            eprintln!("usage: hf-sync <build|push> (got {other:?})");
            process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://raw.githubusercontent.com/foolish-dev/foolish-dev/main/";

    fn staged() -> String {
        transform_body(include_str!("../../../README.md"), BASE)
    }

    #[test]
    fn drops_theme_conditional_markup() {
        let out = staged();
        assert!(!out.contains("#gh-light-mode-only"), "light lines removed");
        assert!(
            !out.contains("#gh-dark-mode-only"),
            "dark fragment stripped"
        );
    }

    #[test]
    fn absolutises_relative_assets_only() {
        let out = staged();
        // Relative banner ref rewritten to the raw base.
        assert!(out.contains(&format!("{BASE}assets/banner-light.svg")));
        // No repo-relative asset refs survive.
        assert!(!out.contains("](assets/"));
    }

    #[test]
    fn preserves_anchors_verbatim() {
        let out = staged();
        // BMC slug must keep its trailing 'm'; the coffee asset is absolutised.
        assert!(out.contains("buymeacoffee.com/cardoffoolm"));
        assert!(out.contains(&format!("{BASE}assets/coffee.svg")));
        assert!(out.contains("ollama run hf.co/FoolDev/Janus-35B:Q4_K_M"));
    }

    #[test]
    fn frontmatter_helpers_roundtrip() {
        let doc = "---\ntitle: X\npinned: true\n---\nbody\nmore\n";
        assert_eq!(
            extract_leading_frontmatter(doc),
            "---\ntitle: X\npinned: true\n---\n"
        );
        assert_eq!(strip_leading_frontmatter(doc), "body\nmore\n");
        // No frontmatter: extract is empty, strip is identity.
        assert_eq!(extract_leading_frontmatter("plain\n"), "");
        assert_eq!(strip_leading_frontmatter("plain\n"), "plain\n");
    }
}
