# @farmfe/plugin-worker

A web worker script can be imported using new Worker() and new SharedWorker().(Inspired by [vite](https://github.com/vitejs/vite))

## Installation

```bash
npm i -D @farmfe/plugin-worker
```

## Usage

Create a `farm.config.js` [configuration file](https://www.farmfe.org/docs/config/configuring-farm) and import the plugin:

```ts
import { defineConfig } from '@farmfe/core';
import worker from '@farmfe/plugin-worker';
export default defineConfig({
  plugins: [
    worker(),
  ],
});
```

## 🚧 通过构造器导入

一个 Web Worker 可以使用 [`new Worker()`](https://developer.mozilla.org/zh-CN/docs/Web/API/Web_Workers_API/Using_web_workers) 和 [`new SharedWorker()`](https://developer.mozilla.org/zh-CN/docs/Web/API/SharedWorker) 导入。与 worker 后缀相比，这种语法更接近于标准，是创建 worker 的 推荐 方式。

```ts
const worker = new Worker(new URL('./worker.js', import.meta.url))

```

worker 构造函数会接受可以用来创建 “模块” worker 的选项：

```ts
const worker = new Worker(new URL('./worker.js', import.meta.url), {
  type: 'module',
})
```

## 带有查询后缀的导入

你可以在导入请求上添加 `?worker` 或 `?sharedworker` 查询参数来直接导入一个 web worker 脚本。默认导出会是一个自定义 worker 的构造函数：

```ts
import MyWorker from './worker?worker'

const worker = new MyWorker()
```

这个 worker 脚本也可以使用 ESM import 语句而不是 importScripts()。注意：在开发时，这依赖于 浏览器原生支持，但是在生产构建中，它会被编译掉。

默认情况下，worker 脚本将在生产构建中编译成单独的 chunk。如果你想将 worker 内联为 base64 字符串，请添加 inline 查询参数：

```ts
import MyWorker from './worker?worker&inline'
```

如果你想要以一个 URL 的形式读取该 worker，请添加 url 这个 query：

```ts
import MyWorker from './worker?worker&url'
```
