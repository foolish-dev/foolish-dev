<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>foolish-dev · profile</title>
  <meta name="description" content="GitHub-style profile card for FoolDev — Janus models, niri-dotfiles, agentic security tooling."/>

  <link rel="preconnect" href="https://fonts.googleapis.com"/>
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
  <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet"/>

  <style>
    /* Tokyo Night palette ------------------------------------------------ */
    :root {
      --bg:        #1a1b26;
      --bg-soft:   #1f2335;
      --bg-card:   #24283b;
      --fg:        #c0caf5;
      --fg-dim:    #9aa5ce;
      --fg-faint:  #565f89;
      --blue:      #7aa2f7;
      --purple:    #bb9af7;
      --cyan:      #7dcfff;
      --green:     #9ece6a;
      --orange:    #ff9e64;
      --red:       #f7768e;
      --border:    #2a2f43;
    }

    * { box-sizing: border-box; }
    html, body {
      margin: 0;
      padding: 0;
      background: var(--bg);
      color: var(--fg);
      font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
      line-height: 1.6;
      font-size: 16px;
    }

    .wrap {
      max-width: 880px;
      margin: 0 auto;
      padding: 2.5rem 1.25rem 4rem;
    }

    a {
      color: var(--blue);
      text-decoration: none;
      transition: color 0.15s;
    }
    a:hover { color: var(--cyan); }

    .center { text-align: center; }
    .row    { display: flex; flex-wrap: wrap; gap: 0.4rem; justify-content: center; align-items: center; }
    .stack  { display: flex; flex-direction: column; gap: 1rem; align-items: center; }

    img.banner {
      max-width: 100%;
      height: auto;
      border-radius: 8px;
    }
    img.badge { height: 22px; }
    img.widget {
      max-width: 100%;
      height: auto;
    }

    hr {
      border: none;
      border-top: 1px solid var(--border);
      margin: 2rem 0;
    }

    /* "cat .profile" code block ----------------------------------------- */
    pre.profile {
      background: var(--bg-card);
      color: var(--fg);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 1rem 1.25rem;
      font-family: 'JetBrains Mono', ui-monospace, monospace;
      font-size: 0.92rem;
      overflow-x: auto;
      margin: 1.5rem 0;
    }
    pre.profile .prompt  { color: var(--green); }
    pre.profile .key     { color: var(--purple); }
    pre.profile .val     { color: var(--cyan); }
    pre.profile .sep     { color: var(--fg-faint); }

    h2 {
      font-family: 'JetBrains Mono', ui-monospace, monospace;
      color: var(--purple);
      font-size: 1.25rem;
      margin: 2.5rem 0 0.25rem;
      letter-spacing: 0.01em;
    }
    .caption {
      color: var(--fg-faint);
      font-family: 'JetBrains Mono', ui-monospace, monospace;
      font-size: 0.85rem;
      margin: 0 0 1rem;
    }

    ul {
      padding-left: 1.25rem;
      margin: 0.5rem 0 1.5rem;
    }
    li {
      margin-bottom: 0.4rem;
    }
    li code, p code {
      background: var(--bg-card);
      padding: 0.05rem 0.35rem;
      border-radius: 4px;
      font-size: 0.88em;
      color: var(--orange);
    }

    /* HF model cards ---------------------------------------------------- */
    .models {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1rem;
      margin: 1rem 0 1.5rem;
    }
    @media (max-width: 600px) {
      .models { grid-template-columns: 1fr; }
    }
    .model-card {
      background: var(--bg-card);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 1rem 1.1rem;
    }
    .model-card a {
      font-family: 'JetBrains Mono', ui-monospace, monospace;
      color: var(--purple);
      font-weight: 600;
    }
    .model-card .meta {
      color: var(--fg-faint);
      font-size: 0.85rem;
      margin: 0.3rem 0 0.6rem;
    }
    .model-card p { margin: 0.4rem 0 0; font-size: 0.92rem; }

    footer {
      margin-top: 3rem;
      color: var(--fg-faint);
      font-family: 'JetBrains Mono', ui-monospace, monospace;
      font-size: 0.85rem;
      text-align: center;
    }
  </style>
</head>
<body>
  <main class="wrap">

    <p class="center">
      <img class="banner"
           src="https://raw.githubusercontent.com/foolish-dev/foolish-dev/main/assets/banner.svg"
           alt="foolish-dev — terminal-themed banner with a mock zsh prompt running whoami"/>
    </p>

    <p class="center">
      <a href="https://github.com/foolish-dev">
        <img src="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=600&size=20&duration=2800&pause=600&color=7AA2F7&center=true&vCenter=true&width=720&lines=%2F%2F+scrollable-tiling+wayland;%2F%2F+offensive+security+operator;%2F%2F+AI-augmented+developer;%2F%2F+terminal-first%2C+keyboard-driven"
             alt="scrollable-tiling wayland · offensive security operator · AI-augmented developer · terminal-first, keyboard-driven"/>
      </a>
    </p>

    <p class="row">
      <img class="badge" src="https://img.shields.io/badge/Arch_Linux-1793D1?style=flat-square&logo=archlinux&logoColor=white" alt="Arch Linux"/>
      <img class="badge" src="https://img.shields.io/badge/Niri-7aa2f7?style=flat-square&logo=wayland&logoColor=white" alt="Niri"/>
      <img class="badge" src="https://img.shields.io/badge/Neovim-57A143?style=flat-square&logo=neovim&logoColor=white" alt="Neovim"/>
      <img class="badge" src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
      <img class="badge" src="https://img.shields.io/badge/Python-3776AB?style=flat-square&logo=python&logoColor=white" alt="Python"/>
      <img class="badge" src="https://img.shields.io/badge/Go-00ADD8?style=flat-square&logo=go&logoColor=white" alt="Go"/>
      <img class="badge" src="https://img.shields.io/badge/Lua-2C2D72?style=flat-square&logo=lua&logoColor=white" alt="Lua"/>
    </p>

    <hr/>

    <pre class="profile"><span class="prompt">~ ❯</span> cat .profile

<span class="key">stack       </span>  <span class="val">arch</span> <span class="sep">·</span> <span class="val">niri</span> <span class="sep">·</span> <span class="val">zsh</span> <span class="sep">·</span> <span class="val">tmux</span> <span class="sep">·</span> <span class="val">neovim</span>
<span class="key">agents      </span>  <span class="val">opencode</span> <span class="sep">·</span> <span class="val">claude-code</span> <span class="sep">·</span> <span class="val">lm-studio</span> <span class="sep">·</span> <span class="val">hexstrike-ai mcp</span>
<span class="key">discipline  </span>  <span class="val">offensive security</span> <span class="sep">·</span> <span class="val">automation</span> <span class="sep">·</span> <span class="val">LLM tooling</span>
<span class="key">theme       </span>  <span class="val">tokyo night + pywal</span> <span class="sep">(wallpaper-driven)</span>
<span class="key">ethos       </span>  <span class="val">terminal-first, keyboard-driven, fully reproducible</span></pre>

    <h2>Now</h2>
    <p class="caption">// current rotation — what I'm shipping, breaking, and iterating on right now</p>
    <ul>
      <li>Driving <a href="https://opencode.ai">opencode</a> + Claude Code side-by-side as daily coding agents.</li>
      <li>Hardening a <a href="https://github.com/foolish-dev/niri-dotfiles">scrollable-tiling Wayland workstation</a> — Niri, Noctalia, pywal.</li>
      <li>Agentic offensive security — local LLMs into MCP-backed recon via <a href="https://github.com/0x4m4/hexstrike-ai">HexStrike AI</a>.</li>
      <li>Shipping <a href="https://github.com/p-e-w/heretic">heretic</a> abliteration runs to Hugging Face — <a href="https://huggingface.co/FoolDev/janus">Janus</a> (35B-A3B MoE) and <a href="https://huggingface.co/FoolDev/janus-27b">Janus-27B</a> (dense).</li>
      <li>Curating a <a href="https://github.com/foolish-dev/niri-dotfiles/tree/main/wallpapers">Tokyo Night wallpaper collection</a> for pywal-driven theming.</li>
    </ul>

    <h2>On Hugging Face</h2>
    <p class="caption">// models published here, kept in sync with the Modelfile / bridge-files dual setup</p>
    <div class="models">
      <div class="model-card">
        <a href="https://huggingface.co/FoolDev/janus">FoolDev/janus</a>
        <p class="meta">Qwen 3.6 35B-A3B MoE · 3B active · Q4_K_M ~19 GB</p>
        <p>Tools / thinking capabilities wired via root-level <code>template</code> / <code>system</code> / <code>params</code> files.</p>
      </div>
      <div class="model-card">
        <a href="https://huggingface.co/FoolDev/janus-27b">FoolDev/janus-27b</a>
        <p class="meta">Qwen 3.6 27B dense · Q4_K_M ~17 GB · Q3_K_S ~12 GB</p>
        <p>Same bridge-file setup, plus <code>examples/</code> + <code>scripts/</code> tooling layer (smoke + bench + bridge-sync regression guards).</p>
      </div>
    </div>
    <p>Both repos: <code>ollama run hf.co/FoolDev/janus[-27b]:Q4_K_M</code>. Tool calls round-trip end-to-end through <code>/api/chat</code> and <code>/v1/chat/completions</code>.</p>

    <h2>Pinned</h2>
    <p class="caption">// flagship repos — the workstation I live in, and this profile itself</p>
    <p class="row">
      <a href="https://github.com/foolish-dev/niri-dotfiles">
        <img class="widget" src="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=niri-dotfiles&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7" alt="foolish-dev/niri-dotfiles"/>
      </a>
      <a href="https://github.com/foolish-dev/foolish-dev">
        <img class="widget" src="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=foolish-dev&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7" alt="foolish-dev/foolish-dev"/>
      </a>
    </p>

    <h2>Signals</h2>
    <p class="caption">// stats, language mix, trophies, activity graph, and the contribution snake</p>

    <p class="row">
      <img class="widget" src="https://github-readme-stats.vercel.app/api?username=foolish-dev&show_icons=true&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7&include_all_commits=true" alt="GitHub stats" height="170"/>
      <img class="widget" src="https://github-readme-stats.vercel.app/api/top-langs/?username=foolish-dev&layout=compact&theme=tokyonight&hide_border=true&bg_color=00000000&title_color=bb9af7&langs_count=8" alt="top languages" height="170"/>
    </p>

    <p class="center">
      <img class="widget" src="https://github-profile-trophy.vercel.app/?username=foolish-dev&theme=tokyonight&no-frame=true&no-bg=true&row=1&column=7&margin-w=8" alt="GitHub trophies"/>
    </p>

    <p class="center">
      <img class="widget" src="https://github-readme-activity-graph.vercel.app/graph?username=foolish-dev&theme=tokyo-night&bg_color=00000000&color=bb9af7&line=7aa2f7&point=7dcfff&area=true&area_color=7aa2f7&hide_border=true&custom_title=Commit%20activity" alt="commit activity"/>
    </p>

    <p class="center">
      <img class="widget" src="https://raw.githubusercontent.com/foolish-dev/foolish-dev/output/snake.svg" alt="contribution snake"/>
    </p>

    <h2>Reach me</h2>
    <p class="caption">// inbox and profile — open to collabs, security work, and agent-tooling conversations</p>
    <p class="row">
      <a href="mailto:cardoffools@gmail.com">
        <img class="badge" src="https://img.shields.io/badge/cardoffools%40gmail.com-7aa2f7?style=flat-square&logo=gmail&logoColor=white" alt="email"/>
      </a>
      <a href="https://github.com/foolish-dev">
        <img class="badge" src="https://img.shields.io/github/followers/foolish-dev?style=flat-square&logo=github&logoColor=white&label=follow&color=bb9af7" alt="GitHub follow"/>
      </a>
      <a href="https://huggingface.co/FoolDev">
        <img class="badge" src="https://img.shields.io/badge/%F0%9F%A4%97%20FoolDev-7dcfff?style=flat-square&logo=huggingface&logoColor=white" alt="Hugging Face"/>
      </a>
    </p>

    <footer>// keep building, keep breaking</footer>

  </main>
</body>
</html>
