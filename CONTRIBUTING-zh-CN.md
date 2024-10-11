# 贡献指南

非常感谢您对 Farm 插件的贡献, 在您提交 Pull Request 之前, 请先阅读以下指南。

## 目录结构

目前 Farm 插件仓库支持 rust 插件和 JavaScript 插件。 rust 插件全部放置在 rust-plugins 下面
JavaScript 插件全部放置在 js-plugins 下面。同时为了方便管理，项目并存 pnpm monorepo 和 rust monorepo。

## 开发 rust 插件

rust 插件内部结构和 farm rust 提供的模版一致，可以直接使用 cli `pnpm create farm-plugin @farmfe/plugin-xxx --type rust` 工具创建，需要注意的是，cli 创建存在一些冗余文件，需要手动删除。可以参考其他的 rust 插件目录结构。同时需要将下列的 crate 包使用 monorepo 的包

```toml
[workspace.dependencies]
farmfe_core = { version = "0.6.4" }
farmfe_utils = { version = "0.1.5" }
farmfe_toolkit_plugin_types = { version = "0.0.20" }
farmfe_macro_plugin = { version = "0.0.4" }
farmfe_toolkit = "0.0.13"
```

修改已有插件的时候，需要注意版本号的更新。需要在插件的根目录下面的运行 `npx changeset` && `npx changeset version` 来更新版本号。

## 开发 JavaScript 插件

开发 JavaScript 插件的时候，同样可以使用 cli `pnpm create farm-plugin @farmfe/plugin-xxx --type js` 来创建，其他和 rust 插件注意事项一致

## 提交 commit 信息规范

为了提高 CI 的运行效率，尽可能的只发布有变动的包，但 rust 插件和 js 插件的发布流程差异较多，我们通过 commit message 来区分发布的包。提交信息的时候 全局运行 `pnpm commit` 来提交 commit 信息，需要根据提示来选择相应的 scope。

## 提交 Pull Request

**PR title** 规范和 commit message 一致。
