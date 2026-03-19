# ALU-CLI

**Agent Logic Unit** — a lightweight CLI tool for high-speed, deterministic math evaluation. Built so AI agents (Claude, Codex, and others) can offload arithmetic to a reliable evaluator instead of hallucinating results.

[![Crates.io](https://img.shields.io/crates/v/alu.svg)](https://crates.io/crates/alu)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![CI](https://github.com/DakshBardoliwala/ALU-CLI/workflows/CI/badge.svg)](https://github.com/DakshBardoliwala/ALU-CLI/actions)

## Why

LLMs are unreliable at precise arithmetic. ALU-CLI is designed to be invoked as a **skill** by AI agents — giving them a trustworthy math co-processor for real computations.

| Metric | Agent + Python | Agent + ALU-CLI |
|---|---|---|
| **Startup Latency** | ~200–500ms | **< 5ms** |
| **User Friction** | Requires "Y/n" approval | **Zero-touch (pre-authorized)** |
| **Dependencies** | Python runtime + libraries | **Zero (standalone binary)** |
| **Determinism** | Potential syntax/env errors | **Deterministic every time** |

## Install

**Quickest — no installation required:**
```bash
npx alu init
```

**macOS (Homebrew):**
```bash
brew install DakshBardoliwala/tap/alu
alu init
```

**macOS / Linux (shell installer):**
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/DakshBardoliwala/ALU-CLI/releases/latest/download/alu-installer.sh | sh
alu init
```

**Windows (PowerShell):**
```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/DakshBardoliwala/ALU-CLI/releases/latest/download/alu-installer.ps1 | iex"
alu init
```

**Rust developers:**
```bash
cargo install alu
alu init
```

## Agent Integration

Running `alu init` does two things automatically:

1. **Installs the skill** — writes `SKILL.md` into `.claude/skills/alu/`, `.codex/skills/alu/`, and `.agents/skills/alu/` so agents can discover ALU on startup
2. **Pre-authorizes the command** — patches `settings.json` to add `Bash(alu *)` to the allow list, so agents invoke ALU without prompting you for approval on every call

By default, skills are installed globally into your home directory. To install into a specific project instead:

```bash
alu init ./my-project
```

The result: agents gain a zero-friction math co-processor with no manual configuration required.

## Usage

### Evaluate an expression

```bash
alu eval "<expression>"
```

**Examples:**

```bash
alu eval "(45.5 * 1024) / 7.2"
# 6471.111111111111

alu eval "sqrt(144) * 2 + sin(pi/4)"
# 24.707106781186546

alu eval "2^10"
# 1024

alu eval "abs(-42) + ceil(3.2) + floor(7.9)"
# 53

alu eval "ln(10)"
# 2.302585092994046

alu eval "cos(pi) + tan(pi/4)"
# -0.00000000000000011102230246251565
```

### Supported operations (Non Exhaustive List)

| Category | Functions / Operators |
|---|---|
| Arithmetic | `+`, `-`, `*`, `/`, `^` |
| Trigonometry | `sin`, `cos`, `tan`, `asin`, `acos`, `atan` |
| Exponential | `exp`, `ln`, `sqrt` |
| Rounding | `ceil`, `floor`, `abs` |
| Constants | `pi`, `e` |

## License

Apache 2.0
