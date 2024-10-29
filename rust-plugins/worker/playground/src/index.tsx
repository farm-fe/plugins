import React from 'react';
import { createRoot } from 'react-dom/client';
import { Main } from './main';
import TestWorker from "./worker/test.worker?worker"
import VueWorker from "./worker/vue.worker?worker"
import './index.css'

console.log(TestWorker);

const worker = new TestWorker();
const worker2 = new VueWorker();
worker.postMessage([5, 5]);
worker.onmessage = (e) => {
  console.log(e.data);
}
worker2.postMessage([3, 3]);
worker2.onmessage = (e) => {
  console.log(e.data);
}

const container = document.querySelector('#root');
const root = createRoot(container);

root.render(<Main />);
