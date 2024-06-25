import React from 'react';
import { createRoot } from 'react-dom/client';
import { Main } from './main';
import './index.css';

import base from './base.yaml';
// import { answer } from './multi.yaml';
console.log('base answer', base);

const container = document.querySelector('#root');
const root = createRoot(container!);

root.render(<Main />);
