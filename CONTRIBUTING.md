# Contribution Guide

Thank you very much for contributing to the Farm plugins. Please read the following guidelines before submitting a Pull Request.

## Directory Structure

Currently, the Farm plugins repository supports both Rust plugins and JavaScript plugins. Rust plugins are all placed under the rust-plugins directory, and JavaScript plugins are all placed under the js-plugins directory. To facilitate management, the project uses both pnpm monorepo and rust monorepo

## Developing Rust Plugins

The internal structure of Rust plugins is consistent with the template provided by Farm Rust. You can directly use the CLI tool `pnpm create farm-plugin @farmfe/plugin-xxx --type rust` to create a new plugin. Note that the CLI creates some redundant files that need to be manually deleted. You can refer to the directory structure of other Rust plugins. Additionally, you need to use the following crate packages from the monorepo:

```toml
[workspace.dependencies]
farmfe_core = { version = "0.6.4" }
farmfe_utils = { version = "0.1.5" }
farmfe_toolkit_plugin_types = { version = "0.0.20" }
farmfe_macro_plugin = { version = "0.0.4" }
farmfe_toolkit = "0.0.13"
```

When modifying existing plugins, be sure to update the version numbers. Run npx changeset and npx changeset version in the root directory of the plugin to update the version numbers.

## Developing JavaScript Plugins

When developing JavaScript plugins, you can also use the CLI tool `pnpm create farm-plugin @farmfe/plugin-xxx --type js` to create a new plugin. The other considerations are the same as for Rust plugins.

## Commit Message Guidelines

To improve CI efficiency, we aim to release only the packages that have changes. However, the release processes for Rust plugins and JavaScript plugins differ significantly. We use commit messages to distinguish the packages to be released. When committing, run pnpm commit globally to submit the commit message, and follow the prompts to select the appropriate scope.

## Submitting a Pull Request

The **PR title** should follow the same conventions as the commit message.