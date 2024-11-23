# 贡献指南

感谢您对 Farm 插件的贡献！在提交 Pull Request 之前，请仔细阅读以下指南。

## 目录结构

目前，Farm 插件仓库支持 Rust 插件和 JavaScript 插件。Rust 插件位于 `rust-plugins` 目录下，JavaScript 插件则在 `js-plugins` 目录中。为了便于管理，项目同时采用了 `pnpm monorepo` 和 `Rust workspace`。

## 开发 rust 插件

Rust 插件的内部结构与 Farm Rust 提供的模板一致，您可以使用 CLI 工具创建插件：

```bash
pnpm create farm-plugin @farmfe/plugin-xxx --type rust
```
请注意，CLI 创建的项目可能包含一些冗余文件，您需要手动删除。建议参考其他 Rust 插件的目录结构。

同时需要将下列的 crate 包使用 `workspace` 的包:

```toml
[workspace.dependencies]
farmfe_core = { version = "0.6.4" }
farmfe_utils = { version = "0.1.5" }
farmfe_toolkit_plugin_types = { version = "0.0.20" }
farmfe_macro_plugin = { version = "0.0.4" }
farmfe_toolkit = "0.0.13"
```

修改已有插件时，请务必更新版本号。在插件根目录下运行以下命令以更新版本：

```bash
npx changeset
npx changeset version
```


## 开发 JavaScript 插件

JavaScript 插件的开发也可以使用 CLI 工具创建：

```bash
pnpm create farm-plugin @farmfe/plugin-xxx --type js
```
其他注意事项与 Rust 插件相同。

## 提交 commit 信息规范

为了提高 CI 的运行效率，我们尽量只发布有变动的包。Rust 和 JavaScript 插件的发布流程存在差异，因此我们通过 `commit message` 来区分需要发布的包。请使用以下命令全局提交 commit 信息：

```bash
pnpm commit
```
根据提示选择相应的 scope。

## 提交 Pull Request

**PR title** 规范和 commit message 一致。
