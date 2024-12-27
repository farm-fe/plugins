# @farmfe/plugin-component

Modular UI library build plugin for Farm.

## Install

### Plugin

```bash
npm i -D @farmfe/plugin-component
```

### Usage

Via `farm.config.ts`.

```ts
import { defineConfig } from '@farmfe/core';
import Icons from '@farmfe/plugin-plugin';

export default defineConfig({
  plugins: [
      ["@farmfe/plugin-component", {
        /**
         * zie of zooming icon
         * @type {string}
         * @default lib
         */
        lib_dir: 'lib',
        /**
         * @description The components lib directory
         * @type {string}
         */
        library_name: "",
        /**
         * @description The UI library name
         * @type {boolean}
         * @default true
         */
        camel2_dash: true,
        /**
         * @description style lib directory, default "lib"
         * @type {string}
         * @default lib
         */
        style_lib_dir: 'lib',
        /**
         * @description the style library name. e.g. custon-theme =>  custon-theme/index.css
         * @type {string}
         */
        style_library_name: '',
        /**
         * @description custom style path
         * @type {string}
         * @default index.css
         */
        style_library_path: 'index.css',
    }],
  ],
});
```

### Example

#### Default Usage

```ts
export default defineConfig({
  plugins: [
    ['@farmfe/plugin-component', {
      library_name: 'element-ui',
    }]
  ],
});
```

###### Converts

```js
import { SomeComponent } from 'element-ui'
```

###### To

```js
import SomeComponent from 'element-ui/lib/SomeComponent';
import 'element-ui/lib/SomeComponent/index.css';
```

#### Set `lib_dir` Usage

```ts
export default defineConfig({
  plugins: [
    ['@farmfe/plugin-component', {
      library_name: 'element-ui',
      lib_dir: 'es',
    }]
  ],
});
```

###### Converts

```js
import { SomeComponent } from 'element-ui'
```

###### To

```js
import SomeComponent from 'element-ui/es/SomeComponent';
import 'element-ui/lib/SomeComponent/index.css';
```

#### Set `camel2_dash` Usage

```ts
export default defineConfig({
  plugins: [
    ['@farmfe/plugin-component', {
      library_name: 'element-ui',
      lib_dir: 'es',
      camel2_dash: false,
    }]
  ],
});
```

###### Converts

```js
import { SomeComponent } from 'element-ui'
```

###### To

```js
import SomeComponent from 'element-ui/es/someComponent';
import 'element-ui/lib/someComponent/index.css';
```

#### Set `style_lib_dir` Usage

```ts
export default defineConfig({
  plugins: [
    ['@farmfe/plugin-component', {
      library_name: 'element-ui',
      lib_dir: 'es',
      camel2_dash: false,
      style_lib_dir: 'lib',
    }]
  ],
});
```

###### Converts

```js
import { SomeComponent } from 'element-ui'
```

###### To

```js
import SomeComponent from 'element-ui/es/someComponent';
import 'element-ui/lib/someComponent/index.css';
```

#### Set `style_library_name` Usage

```ts
export default defineConfig({
  plugins: [
    ['@farmfe/plugin-component', {
      library_name: 'element-ui',
      lib_dir: 'es',
      camel2_dash: false,
      style_lib_dir: 'lib',
      style_library_name: 'theme-default',
    }]
  ],
});
```

###### Converts

```js
import { SomeComponent } from 'element-ui'
```

###### To

```js
import SomeComponent from 'element-ui/es/someComponent';
import 'element-ui/lib/theme-default/someComponent/index.css';
```

#### Set `style_library_path` Usage

```ts
export default defineConfig({
  plugins: [
    ['@farmfe/plugin-component', {
      library_name: 'element-ui',
      lib_dir: 'es',
      camel2_dash: false,
      style_lib_dir: 'lib',
      style_library_name: 'theme-default',
      style_library_path: 'style/index.css'
    }]
  ],
});
```

###### Converts

```js
import { SomeComponent } from 'element-ui'
```

###### To

```js
import SomeComponent from 'element-ui/es/someComponent';
import 'element-ui/lib/theme-default/someComponent/style/index.css';
```