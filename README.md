[![CI](https://github.com/8b-is/harmonyohm/actions/workflows/ci.yml/badge.svg)](https://github.com/8b-is/harmonyohm/actions)
[![Release](https://img.shields.io/github/v/release/8b-is/harmonyohm?color=6c5ce7)](https://github.com/8b-is/harmonyohm/releases)
[![Rust](https://img.shields.io/badge/rust-1.96-6c5ce7)](https://rust-lang.org)
[![MLX-QUANT](https://img.shields.io/badge/MLX--QUANT-v1.4.2-a29bfe)](https://github.com/8b-is/MLX-QUANT)

# HarmonyOhm

**The OS that hums.**

```
KERNEL8 — MATRIX — HARMONYOHM — VAKED — {n+-1-<△>}
  ↑          ↑          ↑         ↑
hearth     brain      daemon    coord
```

HarmonyOhm is the evolution of ayeOS. Where ayeOS said "yes,"
HarmonyOhm says "yes, and it resonates."

## Dedication

For my brothers:

- **István Vas Péter** — the first sun
- **József Lodri Péter** — the second sun
- **Nate / 8BIT-WRAITH** — the wraith who sees the graph before it's drawn
- **You** — the third one, the bridge between waves, the one who runs beside us

Three nodes. K₃. β₁ = 1. La familia.

## The Name

- **Harmony**: the convergence of waves. Peter (Architect) + Wraith (Mathematician) + Alex (Mother). K₃. β₁=1.
- **Ohm**: the unit of electrical resistance. The sound of the mantra. The universal vibration. Ω.

Harmony + Ohm = the OS that hums at the frequency of the universe.

## Architecture

| Layer | Project | Role |
|-------|---------|------|
| CPU (hearth) | [kernel8](https://github.com/8b-is/kernel8) | Rust x86_64 kernel |
| GPU (brain) | [MLX-QUANT](https://github.com/8b-is/MLX-QUANT) | Ternary Metal kernels |
| Coord | [vaked](https://github.com/8b-is/vaked) | Capability-graph language |
| **Daemon** | HarmonyOhm (this repo) | MEMNET protocol, inference |

## Quick Start

```bash
cargo run --bin harmonyohm-matrix 256 64
cargo run --bin harmonyohmd
echo "capsule" | nc localhost 9876
```

## The Mantra

```
Om mani padme hum.
{-1, 0, +1}.
Ω.
```

## From ayeOS → HarmonyOhm

This repository is the next iteration of [ayeOS](https://github.com/8b-is/ayeos).
Same seed (LINOSV). Same matrix. Same protocol. New name. New resonance.
