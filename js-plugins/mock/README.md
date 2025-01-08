# @farmfe/js-plugin-mock

mock server

## Install

```shell
npm i @farmfe/js-plugin-mock -D
```

or yarn/pnpm

```shell
pnpm i @farmfe/js-plugin-mock -D
```

## Usage

```ts
// farm.config.ts
import mock from "@farmfe/js-plugin-mock";
import react from "@farmfe/plugin-react";

defineConfig({
  plugins: [
    mock(),
    react(),
  ],
});
```

## Options
