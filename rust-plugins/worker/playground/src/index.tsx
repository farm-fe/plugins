import { createRoot } from 'react-dom/client';
import { Main } from './main';
import TestWorker from "./worker/test.worker?worker"
import ComlinkWorker from "./worker/comlink.worker?worker"
import './index.css'
import { MyWorker } from './worker/comlink.worker.ts';

// console.log(TestWorker);
// const worker = new TestWorker();
const worker = new Worker(new URL("/src/worker/test.worker.ts",import.meta.url));
worker.postMessage([5, 5]);
worker.onmessage = (e) => {
  console.log(e.data);
}
const worker2 = new Worker(new URL("./worker/vue.worker.ts",import.meta.url))


worker2.postMessage([2, 3]);
worker2.onmessage = (e) => {
  console.log(e.data);
}

const comlinkWorker = (new ComlinkWorker() as unknown as MyWorker );
comlinkWorker.add(1, 2).then((res) => {
  console.log(res); // 3
})

const container = document.querySelector('#root');
const root = createRoot(container!);

root.render(<Main />);
