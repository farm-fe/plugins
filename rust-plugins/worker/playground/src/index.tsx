import React from 'react';
import { createRoot } from 'react-dom/client';
import { Main } from './main';
import TestWorker from "./worker/test.worker?worker"
import './index.css'

const worker = new TestWorker();
worker.postMessage([5, 5]);
worker.onmessage = (e) => {
  console.log("worker response: ", e.data);
}

const container = document.querySelector('#root');
const root = createRoot(container);

root.render(<Main />);
