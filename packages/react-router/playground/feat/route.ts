import * as route57321 from '../routes/concerts_.your.tsx'
import * as route86759 from '../routes/concerts_.mine.tsx'
import * as route26116 from '../routes/concerts.tsx'
import * as route19440 from '../routes/concerts.trending.tsx'
import * as route82174 from '../routes/concerts._index.tsx'
import * as route76427 from '../routes/concerts.$city.tsx'
import * as route88004 from '../routes/concerts.$.tsx'
import * as route82398 from '../routes/auth.$test.tsx'
import * as route29280 from '../routes/auth.$.tsx'
import * as route20320 from '../routes/_index.tsx'
import * as route71204 from '../routes/_auth.login.tsx'


function moduleFactory(module) {
  const { default: Component, clientLoader: loader, clientAction: action, loader: _loader, action: _action, Component: _Component, ...rest } = module;
  return { Component, loader, action, ...rest };
}

export const routes = [
  { "path": "notes", "lazy": () => import('../routes/notes.lazy.tsx').then(moduleFactory) },
  { "path": "concerts/your", ...moduleFactory(route57321) },
  { "path": "concerts/mine", ...moduleFactory(route86759) },
  {
    "path": "concerts", ...moduleFactory(route26116),
    "children": [
      { "path": "trending", ...moduleFactory(route19440) },
      { "index": true, ...moduleFactory(route82174) },
      { "path": ":city", ...moduleFactory(route76427) },
      { "path": "*", ...moduleFactory(route88004) }
    ]
  },
  {
    "path": "auth", "lazy": () => import('../routes/auth.lazy.tsx').then(moduleFactory),
    "children": [
      {
        "path": "recover", "lazy": () => import('../routes/auth.recover/route.lazy.tsx').then(moduleFactory),
        "children": [{ "path": "test", "lazy": () => import('../routes/auth.recover.test.lazy.tsx').then(moduleFactory) }]
      },
      {
        "path": "new", "children": [{
          "path": "without",
          "children": [{ "path": "parent", "lazy": () => import('../routes/auth.new.without.parent.lazy.tsx').then(moduleFactory) }]
        }]
      },
      { "path": "forgot-pass", "lazy": () => import('../routes/auth.forgot-pass.lazy.tsx').then(moduleFactory) },
      {
        "path": "deploy", "lazy": () => import('../routes/auth.deploy.lazy.tsx').then(moduleFactory),
        "children": [{ "path": "test", "lazy": () => import('../routes/auth.deploy.test.lazy.tsx').then(moduleFactory) }]
      },
      { "index": true, "lazy": () => import('../routes/auth._index.lazy.tsx').then(moduleFactory) },
      { "path": ":test", ...moduleFactory(route82398) }, { "path": "*", ...moduleFactory(route29280) }
    ]
  },
  { "index": true, ...moduleFactory(route20320) },
  {
    "lazy": () => import('../routes/_auth.lazy.tsx').then(moduleFactory),
    "children": [{ "path": "login", ...moduleFactory(route71204) }]
  },
  {
    "path": ":lang?", "children":
      [{ "path": "dashboard", "lazy": () => import('../routes/($lang).dashboard.lazy.tsx').then(moduleFactory) }]
  }]

