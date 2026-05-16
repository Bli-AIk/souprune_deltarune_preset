# souprune_deltarune_preset

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_deltarune_preset.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_deltarune_preset.svg"/>

**souprune_deltarune_preset** is the maintained Deltarune-style reusable library mod for SoupRune.

| English | Simplified Chinese |
|---------|--------------------|
| English | [简体中文](./readme_zh-hans.md) |

## Introduction

This project provides reusable SoupRune content for Deltarune-style games.
The first milestone focuses on the overworld dark menu: party status, ITEM / STORAGE / KEYITEM navigation, DR-style 640x480 coordinates, and minimal runtime glue for dependent mods.

This is a project-level library mod, not a compiled framework layer and not a standalone game. A project mod should depend on it and provide its own entry scene or smoke-test content.

## How to Use

The recommended path is to clone the main SoupRune repository and initialize submodules:

```bash
git clone https://github.com/Bli-AIk/souprune.git
cd souprune
git submodule update --init --recursive
```

This repository is mounted by the main project at:

```text
projects/deltarune_preset
```

Project mods can depend on it from their `mod.toml`:

```toml
[dependencies]
deltarune_preset = "0.1.0"
```

## Current Scope

- Deltarune-style overworld dark menu.
- Party facts for up to three visible members.
- ITEM / STORAGE / KEYITEM category and list navigation.
- 640x480, top-left-origin, Y-down View coordinate space.

Battle UI, battle commands, equipment, TP, Magic, shops, and complete item effects are intentionally outside this milestone.

## License and Asset Notice

The original code, configuration, and scripts in this repository are licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

This license applies only to original repository code, configuration, scripts, and other original contributions.
Undertale/Deltarune-related characters, names, visual assets, audio assets, and other original-game materials remain the property of their respective rights holders.
This repository is a fangame-development preset and does not grant any rights to Undertale or Deltarune assets.
