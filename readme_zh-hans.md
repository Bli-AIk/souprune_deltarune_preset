# souprune_deltarune_preset

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_deltarune_preset.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_deltarune_preset.svg"/>

**souprune_deltarune_preset** 是 SoupRune 维护中的 Deltarune 风格可复用预设项目。

| 英语 | 简体中文 |
|------|----------|
| [English](./readme.md) | 简体中文 |

## 简介

本项目为 Deltarune 风格游戏提供可复用的 SoupRune 内容。
第一阶段聚焦 overworld dark menu：队伍状态、ITEM / STORAGE / KEYITEM 导航、DR 风格 640x480 坐标，以及供依赖 mod 加载的最小 runtime glue。

这是一个库 mod，不是独立游戏。项目 mod 应依赖它，并提供自己的入口场景或 smoke-test 内容。

## 使用方法

推荐克隆主 SoupRune 仓库并初始化子模块：

```bash
git clone https://github.com/Bli-AIk/souprune.git
cd souprune
git submodule update --init --recursive
```

主项目会将此仓库挂载到：

```text
projects/deltarune_preset
```

项目 mod 可以在自己的 `mod.toml` 中依赖它：

```toml
[dependencies]
deltarune_preset = "0.1.0"
```

## 当前范围

- Deltarune 风格 overworld dark menu。
- 最多三名可见 party 成员的状态 facts。
- ITEM / STORAGE / KEYITEM 分类与列表导航。
- 640x480、左上原点、Y 向下的 View 坐标空间。

战斗 UI、战斗指令、装备、TP、Magic、商店和完整道具效果不属于本阶段。

## 许可证与素材说明

本仓库中的原创代码、配置和脚本采用以下任一许可证：

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) 或 [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) 或 [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

任选其一。

上述许可证仅适用于本仓库中的原创代码、配置、脚本和其他原创贡献。
与 Undertale/Deltarune 相关的角色、名称、视觉素材、音频素材和其他原作材料，其权利仍归各自权利方所有。
本仓库是同人游戏开发预设，不授予任何 Undertale 或 Deltarune 素材权利。
