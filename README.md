<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/foolish-dev/foolish-dev/main/assets/banner.svg"/>
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/foolish-dev/foolish-dev/main/assets/banner-light.svg"/>
    <img src="assets/banner.svg" alt="foolish-dev — terminal-themed banner with a mock zsh prompt running whoami"/>
  </picture>
</p>

<p align="center">
  <a href="https://github.com/foolish-dev">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=600&size=20&duration=2800&pause=600&color=7AA2F7&center=true&vCenter=true&width=720&lines=%2F%2F+scrollable-tiling+wayland;%2F%2F+offensive+security+operator;%2F%2F+AI-augmented+developer;%2F%2F+terminal-first%2C+keyboard-driven"/>
      <source media="(prefers-color-scheme: light)" srcset="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=600&size=20&duration=2800&pause=600&color=2E7DE9&center=true&vCenter=true&width=720&lines=%2F%2F+scrollable-tiling+wayland;%2F%2F+offensive+security+operator;%2F%2F+AI-augmented+developer;%2F%2F+terminal-first%2C+keyboard-driven"/>
      <img src="https://readme-typing-svg.demolab.com?font=JetBrains+Mono&weight=600&size=20&duration=2800&pause=600&color=7AA2F7&center=true&vCenter=true&width=720&lines=%2F%2F+scrollable-tiling+wayland;%2F%2F+offensive+security+operator;%2F%2F+AI-augmented+developer;%2F%2F+terminal-first%2C+keyboard-driven" alt="scrollable-tiling wayland · offensive security operator · AI-augmented developer · terminal-first, keyboard-driven"/>
    </picture>
  </a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Arch_Linux-1793D1?style=flat-square&logo=archlinux&logoColor=white" alt="Arch Linux"/>
  <img src="https://img.shields.io/badge/Niri-7aa2f7?style=flat-square&logo=wayland&logoColor=white" alt="Niri"/>
  <img src="https://img.shields.io/badge/Neovim-57A143?style=flat-square&logo=neovim&logoColor=white" alt="Neovim"/>
  <img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/Python-3776AB?style=flat-square&logo=python&logoColor=white" alt="Python"/>
  <img src="https://img.shields.io/badge/Go-00ADD8?style=flat-square&logo=go&logoColor=white" alt="Go"/>
  <img src="https://img.shields.io/badge/Lua-2C2D72?style=flat-square&logo=lua&logoColor=white" alt="Lua"/>
</p>

---

```text
~ ❯ cat .profile

stack         arch · niri · zsh · tmux · neovim
agents        opencode · claude-code · lm-studio · hexstrike-ai mcp
discipline    offensive security · automation · LLM tooling
theme         tokyo night + pywal (wallpaper-driven)
mirror        github → huggingface.co/FoolDev (auto-synced)
ethos         terminal-first, keyboard-driven, fully reproducible
```

### Now

<sub>// current rotation — what I'm shipping, breaking, and iterating on right now</sub>

- Driving [opencode](https://opencode.ai) + Claude Code side-by-side as daily coding agents.
- Hardening a [scrollable-tiling Wayland workstation](https://github.com/foolish-dev/niri-dotfiles) — Niri, Noctalia, pywal.
- Agentic offensive security — local LLMs into MCP-backed recon via [HexStrike AI](https://github.com/0x4m4/hexstrike-ai).
- Shipping [heretic](https://github.com/p-e-w/heretic) abliteration runs to Hugging Face — [Janus](https://huggingface.co/FoolDev/janus) (35B-A3B MoE) and [Janus-27B](https://huggingface.co/FoolDev/janus-27b) (dense).
- Curating a [Tokyo Night wallpaper collection](https://github.com/foolish-dev/niri-dotfiles/tree/main/wallpapers) for pywal-driven theming.

### On Hugging Face

<sub>// models published here, kept in sync with the Modelfile / bridge-files dual setup</sub>

- **[FoolDev/janus](https://huggingface.co/FoolDev/janus)** — Janus-35B (Qwen 3.6 35B-A3B MoE, 3B active), Q4_K_M ~19 GB. Tools / thinking capabilities wired via root-level `template` / `system` / `params` files.
- **[FoolDev/janus-27b](https://huggingface.co/FoolDev/janus-27b)** — Janus-27B (Qwen 3.6 27B dense), Q4_K_M ~17 GB and Q3_K_S ~12 GB. Same bridge-file setup, plus an `examples/` + `scripts/` tooling layer (smoke + bench + bridge-sync regression guards).

Both repos: `ollama run hf.co/FoolDev/janus[-27b]:Q4_K_M`. Tool calls round-trip end-to-end through `/api/chat` and `/v1/chat/completions`.

### Pinned

<sub>// flagship repos — the workstation I live in, and this profile itself</sub>

<p align="center">
  <a href="https://github.com/foolish-dev/niri-dotfiles">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=niri-dotfiles&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7"/>
      <source media="(prefers-color-scheme: light)" srcset="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=niri-dotfiles&theme=tokyonight_duo&hide_border=true&bg_color=00000000&icon_color=2e7de9&title_color=9854f1"/>
      <img src="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=niri-dotfiles&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7" alt="foolish-dev/niri-dotfiles — Arch + Niri + Tokyo Night workstation, BlackArch tooling, pywal theming"/>
    </picture>
  </a>
  <a href="https://github.com/foolish-dev/foolish-dev">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=foolish-dev&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7"/>
      <source media="(prefers-color-scheme: light)" srcset="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=foolish-dev&theme=tokyonight_duo&hide_border=true&bg_color=00000000&icon_color=2e7de9&title_color=9854f1"/>
      <img src="https://github-readme-stats.vercel.app/api/pin/?username=foolish-dev&repo=foolish-dev&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7" alt="foolish-dev/foolish-dev — this GitHub profile README repository"/>
    </picture>
  </a>
</p>

### Signals

<sub>// stats, language mix, trophies, activity graph, and the contribution snake</sub>

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github-readme-stats.vercel.app/api?username=foolish-dev&show_icons=true&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7&include_all_commits=true"/>
    <source media="(prefers-color-scheme: light)" srcset="https://github-readme-stats.vercel.app/api?username=foolish-dev&show_icons=true&theme=tokyonight_duo&hide_border=true&bg_color=00000000&icon_color=2e7de9&title_color=9854f1&include_all_commits=true"/>
    <img src="https://github-readme-stats.vercel.app/api?username=foolish-dev&show_icons=true&theme=tokyonight&hide_border=true&bg_color=00000000&icon_color=7aa2f7&title_color=bb9af7&include_all_commits=true" alt="foolish-dev GitHub stats — commits, PRs, issues, stars" height="170"/>
  </picture>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github-readme-stats.vercel.app/api/top-langs/?username=foolish-dev&layout=compact&theme=tokyonight&hide_border=true&bg_color=00000000&title_color=bb9af7&langs_count=8"/>
    <source media="(prefers-color-scheme: light)" srcset="https://github-readme-stats.vercel.app/api/top-langs/?username=foolish-dev&layout=compact&theme=tokyonight_duo&hide_border=true&bg_color=00000000&title_color=9854f1&langs_count=8"/>
    <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=foolish-dev&layout=compact&theme=tokyonight&hide_border=true&bg_color=00000000&title_color=bb9af7&langs_count=8" alt="foolish-dev top languages breakdown" height="170"/>
  </picture>
</p>

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github-profile-trophy.vercel.app/?username=foolish-dev&theme=tokyonight&no-frame=true&no-bg=true&row=1&column=7&margin-w=8"/>
    <source media="(prefers-color-scheme: light)" srcset="https://github-profile-trophy.vercel.app/?username=foolish-dev&theme=flat&no-frame=true&no-bg=true&row=1&column=7&margin-w=8"/>
    <img src="https://github-profile-trophy.vercel.app/?username=foolish-dev&theme=tokyonight&no-frame=true&no-bg=true&row=1&column=7&margin-w=8" alt="foolish-dev GitHub trophies"/>
  </picture>
</p>

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github-readme-activity-graph.vercel.app/graph?username=foolish-dev&theme=tokyo-night&bg_color=00000000&color=bb9af7&line=7aa2f7&point=7dcfff&area=true&area_color=7aa2f7&hide_border=true&custom_title=Commit%20activity"/>
    <source media="(prefers-color-scheme: light)" srcset="https://github-readme-activity-graph.vercel.app/graph?username=foolish-dev&bg_color=00000000&color=9854f1&line=2e7de9&point=007197&area=true&area_color=2e7de9&hide_border=true&custom_title=Commit%20activity"/>
    <img src="https://github-readme-activity-graph.vercel.app/graph?username=foolish-dev&theme=tokyo-night&bg_color=00000000&color=bb9af7&line=7aa2f7&point=7dcfff&area=true&area_color=7aa2f7&hide_border=true&custom_title=Commit%20activity" alt="foolish-dev commit activity graph over time"/>
  </picture>
</p>

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/foolish-dev/foolish-dev/output/snake.svg"/>
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/foolish-dev/foolish-dev/output/snake-light.svg"/>
    <img src="https://raw.githubusercontent.com/foolish-dev/foolish-dev/output/snake.svg" alt="foolish-dev contribution graph animated as a snake eating the contribution dots"/>
  </picture>
</p>

### Reach me

<sub>// inbox and profile — open to collabs, security work, and agent-tooling conversations</sub>

<p align="center">
  <a href="mailto:cardoffools@gmail.com">
    <img src="https://img.shields.io/badge/cardoffools%40gmail.com-7aa2f7?style=flat-square&logo=gmail&logoColor=white" alt="email cardoffools at gmail dot com"/>
  </a>
  <a href="https://github.com/foolish-dev">
    <img src="https://img.shields.io/github/followers/foolish-dev?style=flat-square&logo=github&logoColor=white&label=follow&color=bb9af7" alt="Follow foolish-dev on GitHub"/>
  </a>
  <a href="https://huggingface.co/FoolDev">
    <img src="https://img.shields.io/badge/%F0%9F%A4%97%20FoolDev-7dcfff?style=flat-square&logo=huggingface&logoColor=white" alt="Hugging Face profile FoolDev"/>
  </a>
</p>

<p align="center">
  <sub>// keep building, keep breaking</sub>
</p>
