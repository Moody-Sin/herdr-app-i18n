# herdr-app-i18n

> 为智能体编码与终端工作流而生的原生桌面端应用（全量简体中文本地化分支）。

<p align="center">
  <img src="assets/icons/herdr-app.png" alt="herdr-app logo" width="100" />
</p>

<p align="center">
  <a href="https://herdr.dev">官方主页</a> · <a href="#安装">安装</a> · <a href="#核心特性">特性</a> · <a href="#从源码构建">构建</a> · <a href="https://github.com/herdrdev/herdr-app">上游英文原版</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-666666?labelColor=333333" alt="Apache 2.0 许可证" /></a>
  <a href="https://github.com/herdrdev/herdr-app/releases"><img src="https://img.shields.io/github/v/release/herdrdev/herdr-app?label=upstream&labelColor=333333&color=666666" alt="上游版本" /></a>
</p>

---

## 目录

- [背景与定位](#背景与定位)
- [与上游的关系](#与上游的关系)
- [核心特性](#核心特性)
- [安装方式](#安装方式)
- [快速上手](#快速上手)
  - [空间与窗格](#空间与窗格)
  - [智能体聚合侧边栏](#智能体聚合侧边栏)
  - [Peek 悬浮预览](#peek-悬浮预览)
  - [远程多机器支持](#远程多机器支持)
- [布局与扩展 (Mods)](#布局与扩展-mods)
- [面向智能体](#面向智能体)
- [从源码构建](#从源码构建)
- [许可证](#许可证)

---

## 背景与定位

[Herdr Desktop App](https://github.com/herdrdev/herdr-app) 是由 Herdr 团队全新打造的原生桌面客户端。与早期的纯终端 TUI 版相比，桌面端采用了极致性能的跨平台底层架构：

- **GPUI 渲染引擎**：基于 Zed 同款的 GPU 硬件加速渲染管线（Vulkan / Metal / Direct3D），在高刷与重载下依旧丝滑省电。
- **Ghostty VT 核心**：深度集成 Ghostty 终端虚拟层，具备完备的现代终端协议（真彩、Kitty 图像协议、重排及超低时延）。
- **进程与窗口解耦**：底层依托 [`herdr-core`](https://github.com/herdrdev/herdr-core) 守护进程维持终端与智能体的生命周期。关闭窗口或重启系统，后台智能体与编译任务依然平稳运行。
- **可编程扩展 (Mods)**：内置 QuickJS 引擎，面板、侧边栏、状态栏均由高内聚的 JavaScript 插件驱动，支持秒级热重载与自由定制。

---

## 与上游的关系

本仓库为官方上游 [`herdrdev/herdr-app`](https://github.com/herdrdev/herdr-app) 的 **简体中文全量本地化分支**：

1. **界面全量中文**：覆盖智能体列表、空间管理、Git 变更、文件浏览器、设置面板、快捷键说明及核心命令。
2. **完全独立维护**：本分支不向上游提 PR，专门面向中文开发者社区维护高质量汉化与同步。
3. **上游持续跟进**：定期拉取 upstream 最新特性与 Bug 修复，保障版本功能同步。

---

## 核心特性

- **智能体统一看板**：全局聚合多工作区、多窗格、甚至智能体派生出的所有 Worker。实时清晰展示状态：`等待处理`（需要确认）、`运行中`（正在思考或执行）、`已完成`（已出结果）、`空闲`。
- **Peek 原生悬浮速览**：当智能体在文本中提到某个终端编号（如 `t3` 或 `remote/t7`）时，按住 `Ctrl`（macOS 为 `Cmd`）点击终端地址，即可弹出悬浮小窗直接查看与交互，无需切换当前工作区。
- **拖拽自由布局**：支持类似现代 IDE 的自由拖拽分栏、停靠、交换及最大化聚焦。
- **远程无缝漫游**：直读 `~/.ssh/config`，一键连接远端主机。远端终端始终保存在远端核心中，本地关机断网不中断远端任务。

---

## 安装方式

### 官方脚本安装

#### Linux 与 macOS

```sh
curl -fsSL https://releases.herdr.dev/app/install.sh | sh
```

> Linux 需要图形桌面环境与支持 Vulkan 的驱动；macOS 目前提供 Apple Silicon 原生构建，Intel Mac 可由源码构建。

#### Windows (PowerShell)

在 64 位 Windows 10 (1803+) 或 Windows 11 环境下运行：

```powershell
irm https://releases.herdr.dev/app/install.ps1 | iex
```

安装程序会自动部署到 `%LOCALAPPDATA%\Programs\herdr-app` 并创建开始菜单快捷方式，同时按需自动安装匹配的 `herdr-core`。

---

## 快速上手

### 空间与窗格
- **空间 (Space)**：对应一个项目或仓库的工作区，包含独立的标签页与窗格。
- **分栏与调整**：鼠标悬停在窗格顶部把手可拖动重新排列；右键菜单支持快速“向右分栏”、“向下分栏”或“最大化窗格”。

### 智能体聚合侧边栏
- 侧边栏实时汇总当前运行的智能体；
- 支持**置顶**重要智能体；
- 支持切换**按空间分组**或**按优先级排序**（优先呈现等待人类回复的智能体）。

### Peek 悬浮预览
- 终端文本中出现的终端地址（如 `t1`、`server/t2`）具有交互感知能力；
- 按住 `Ctrl` 点击直接打开悬浮速览；可一键将速览窗格“提升为标签页”。

### 远程多机器支持
- 在 `设置 → 机器` 或侧边栏中添加 SSH 主机；
- 远程空间完全在远端执行，断网与休眠不丢失现场。

---

## 布局与扩展 (Mods)

桌面端的所有界面面板均作为 **Mod**（JavaScript 小程序）实现，位于用户配置目录下的 `mods/` 文件夹中：
- 布局结构保存在 `layouts/` 目录下；
- 保存自定义 Mod 或主题后，窗口**即时热重载**，无需重启；
- 按 `Ctrl+Shift+P`（macOS 为 `Cmd+Shift+P`）可唤起命令面板，查找与执行任意操作。

---

## 面向智能体

桌面端内置面向 AI 智能体的交互与引导协议：

```sh
herdr-app --skill
```

该命令会输出专门教导智能体如何控制和操作桌面窗口的指南与接口契约。智能体亦可使用以下命令检查窗口状态：

```sh
herdr-app ls
herdr-app status
herdr-app commands
```

---

## 从源码构建

编译构建需要 **Rust 1.95+** 与 **Zig 0.16.0**：

```sh
git clone --recurse-submodules https://github.com/Moody-Sin/herdr-app-i18n.git
cd herdr-app-i18n
cargo build --release
```

运行单元测试与校验：

```sh
cargo test
bun test scripts/files.test.ts scripts/picker.test.ts scripts/agents.test.ts scripts/spaces.test.ts
```

---

## 许可证

本项目基于 [Apache-2.0](LICENSE) 协议开源。
