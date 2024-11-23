# @farmfe/plugin-svgr

react svg component generator

## Installation

```bash
npm i -D @farmfe/plugin-svgr
```

## Usage

Create a `farm.config.js` [configuration file](https://www.farmfe.org/docs/config/configuring-farm) and import the plugin:

```ts
import { defineConfig } from "@farmfe/core";
import svgr from "@farmfe/plugin-svgr";
export default defineConfig({
  plugins: [svgr()],
});
```
