import React, { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css'
// @ts-ignore
import  { routes } from "virtual:react-routes";

import { createBrowserRouter, RouterProvider } from 'react-router-dom';
console.log(routes);
const router = createBrowserRouter(routes, {
  future: {
    v7_fetcherPersist: true,
    v7_normalizeFormMethod: true,
    v7_partialHydration: true,
    v7_relativeSplatPath: true,
    v7_skipActionErrorRevalidation: true,
  },
})

// const container = document.querySelector('#root');
// const root = createRoot(container);

// root.render(<Main />);
export default createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
)
