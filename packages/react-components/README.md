# @farmfe/plugin-react-components

On-demand components auto importing for React.

## Installation

```bash
npm i @farmfe/plugin-react-components
```

## Usage

`@farmfe/plugin-react-components` is a Rust plugin, you only need to configure its package name in `plugins` field in `farm.config.ts`.

```ts {4}
import { UserConfig } from '@farmfe/core';

const config: UserConfig = {
  plugins: ['@farmfe/plugin-react-components', { /** options here */}]
}
```

## Features

- 💚 Supports React out-of-the-box.
- ✨ Supports both components and directives.
- 🏝 Tree-shakable, only registers the components you use.
- 🪐 Folder names as namespaces.
- 🦾 Full TypeScript support.
- 🌈 [Built-in resolvers](#importing-from-ui-libraries) for popular UI libraries.

## Usage

Use components in templates as you would usually do, it will import components on demand, and there is no `import` and `component registration` required anymore! If you register the parent component asynchronously (or lazy route), the auto-imported components will be code-split along with their parent.

It will automatically turn this

```tsx
export function Main() {
  return <HelloWorld msg="Hello React + Farm" />
}
```

into this

```tsx
import HelloWorld from './src/components/HelloWorld'

export function Main() {
  return <HelloWorld msg="Hello React + Farm" />
}
```

> **Note**
> By default this plugin will import components in the `src/components` path. You can customize it using the `dirs` option.

## TypeScript

To get TypeScript support for auto-imported components.

```ts
Components({
  dts: true, // enabled by default if `typescript` is installed
})
```

Once the setup is done, a `components.d.ts` will be generated and updates automatically with the type definitions. Feel free to commit it into git or not as you want.

> **Make sure you also add `components.d.ts` to your `tsconfig.json` under `include`.**

## Importing from UI Libraries

We have several built-in resolvers for popular UI libraries like **Ant Design**, **Arco Design**, and **Material UI**, where you can enable them by:

Supported Resolvers:

- [Ant Design](https://ant.design/)
- [Arco Design](https://arco.design/react/docs/start)
- [Material UI](https://mui.com/)

```ts
// farm.config.js

import { UserConfig } from '@farmfe/core';

const config: UserConfig = {
  plugins: ['@farmfe/plugin-react-components', {
        local: true,
        resolvers:[
          {
            module: "antd",
            prefix: "Ant"
          },
          {
            module:"@arco-design/web-react",
            prefix: "Arco",
            import_style: true // style/index.js
          }
        ]
  }]
}
```

## Configuration

The following show the default values of the configuration
<strike>component</strike>

```ts
{
  // relative paths to the directory to search for components.
  dirs: ['src/components'],
  
  // resolvers for custom components.
  resolvers: [],

  /**
   * Components are introduced with Absolute or Relative path.
   *
   * @default Absolute
   */
  import_mode: "Absolute"

  /**
   * Is it valid for local components
   *
   * @default true
   */
  local: true,

  /**
   * import style `style/index.js` , also accepts a path for custom path (<Component>/**) with components
   *
   * @default false
   */
  importStyle?: boolean | string 

  // generate `components.d.ts` global declarations,
  // also accepts a path for custom filename
  // default: `true` if package typescript is installed
  dts: true,

  // Filters for transforming targets (components to insert the auto import)
  // Note these are NOT about including/excluding components registered - use `Regex` for that
  include: ["src/components"],
  exclude: ["node_modules"],
}
```
