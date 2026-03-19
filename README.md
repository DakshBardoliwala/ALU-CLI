# ALU-CLI

**Agent Logic Unit** — a lightweight CLI tool for high-speed, deterministic math evaluation. Built so AI agents (Claude, Codex, and others) can offload arithmetic to a reliable evaluator instead of hallucinating results.

## Why

LLMs are unreliable at precise arithmetic. ALU-CLI is designed to be invoked as a **skill** by AI agents — giving them a trustworthy math co-processor for real computations.

## Install

```bash
git clone https://github.com/your-username/alu-cli
cd alu-cli
cargo build --release
# Add to PATH
cp target/release/alu /usr/local/bin/alu
```

Then register it as an agent skill:

```bash
alu init
```

This installs `SKILL.md` into `~/.claude/skills/alu/`, `~/.codex/skills/alu/`, and `~/.agents/skills/alu/`, and adds `Bash(alu *)` to the agent permission allowlists in their respective `settings.json` files.

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

## Agent Integration

After running `alu init`, agents can discover and invoke ALU as a skill. Claude, for example, will find the skill definition at `~/.claude/skills/alu/SKILL.md` and use `alu eval` for any arithmetic it would otherwise compute inline.

The `init` command also patches `settings.json` to pre-authorize `Bash(alu *)`, so agents don't need to prompt for approval on each invocation.

## Requirements

- Rust 1.94+

## License

Apache 2.0
