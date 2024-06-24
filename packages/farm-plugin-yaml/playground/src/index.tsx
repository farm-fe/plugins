import React from 'react';
import { createRoot } from 'react-dom/client';
import { Main } from './main';
import './index.css';

import base from './base.yaml';
import { answer } from './base.yaml';
console.log('base answer', base, answer);

const container = document.querySelector('#root');
const root = createRoot(container!);

root.render(<Main />);
