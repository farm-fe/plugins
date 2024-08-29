import * as farmfe_plugin_react_router_2e71f9f8 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/concerts_.mine.tsx';
import * as farmfe_plugin_react_router_a5020bac from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/concerts_.your.tsx';
import * as farmfe_plugin_react_router_0da27a10 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.recover/store.tsx';
import * as farmfe_plugin_react_router_c3f09479 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.$test.tsx';
import * as farmfe_plugin_react_router_c1095542 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.recover/index.tsx';
import * as farmfe_plugin_react_router_fa180d5e from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.$.tsx';
import * as farmfe_plugin_react_router_090cd3b4 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/concerts.tsx';
import * as farmfe_plugin_react_router_dae204bb from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/concerts._index.tsx';
import * as farmfe_plugin_react_router_870d5865 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/concerts.$city.tsx';
import * as farmfe_plugin_react_router_23551dfd from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/concerts.$.tsx';
import * as farmfe_plugin_react_router_18ababe8 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/concerts.trending.tsx';
import * as farmfe_plugin_react_router_4fa9f607 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/_index.tsx';
import * as farmfe_plugin_react_router_339bb3b2 from '/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/_auth.login.tsx';


const routes = [
  {
    "children": [
      {
        "path": "dashboard",
        "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/(lang).dashboard.lazy.tsx').then(adapter))$"
      }
    ],
    "path": ":lang?",
    "component": "lazy(() => import('').then(adapter))"
  },
  {
    "children": [
      {
        "path": "mine",
        "component": "...adapter(farmfe_plugin_react_router_2e71f9f8)"
      },
      {
        "path": "your",
        "component": "...adapter(farmfe_plugin_react_router_a5020bac)"
      }
    ],
    "path": "concerts_",
    "component": "lazy(() => import('').then(adapter))"
  },
  {
    "children": [
      {
        "path": "recover/store",
        "component": "...adapter(farmfe_plugin_react_router_0da27a10)"
      },
      {
        "index": true,
        "path": "",
        "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth._index.lazy.tsx').then(adapter))"
      },
      {
        "children": [
          {
            "path": "test",
            "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.recover.test.lazy.tsx').then(adapter))"
          }
        ],
        "path": "recover",
        "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.recover/route.lazy.tsx').then(adapter))"
      },
      {
        "path": ":test",
        "component": "...adapter(farmfe_plugin_react_router_c3f09479)"
      },
      {
        "children": [
          {
            "children": [
              {
                "path": "parent",
                "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.new.without.parent.lazy.tsx').then(adapter))"
              }
            ],
            "path": "without",
            "component": "lazy(() => import('').then(adapter))"
          }
        ],
        "path": "new",
        "component": "lazy(() => import('').then(adapter))"
      },
      {
        "path": "recover/index",
        "component": "...adapter(farmfe_plugin_react_router_c1095542)"
      },
      {
        "path": "*",
        "component": "...adapter(farmfe_plugin_react_router_fa180d5e)"
      },
      {
        "children": [
          {
            "path": "test",
            "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.deploy.test.lazy.tsx').then(adapter))"
          }
        ],
        "path": "deploy",
        "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.deploy.lazy.tsx').then(adapter))"
      },
      {
        "path": "forgot-pass",
        "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.forgot-pass.lazy.tsx').then(adapter))"
      }
    ],
    "path": "auth",
    "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/auth.lazy.tsx').then(adapter))"
  },
  {
    "children": [
      {
        "index": true,
        "path": "",
        "component": "...adapter(farmfe_plugin_react_router_dae204bb)"
      },
      {
        "path": ":city",
        "component": "...adapter(farmfe_plugin_react_router_870d5865)"
      },
      {
        "path": "*",
        "component": "...adapter(farmfe_plugin_react_router_23551dfd)"
      },
      {
        "path": "trending",
        "component": "...adapter(farmfe_plugin_react_router_18ababe8)"
      }
    ],
    "path": "concerts",
    "component": "...adapter(farmfe_plugin_react_router_090cd3b4)"
  },
  {
    "index": true,
    "path": "",
    "component": "...adapter(farmfe_plugin_react_router_4fa9f607)"
  },
  {
    "children": [
      {
        "path": "login",
        "component": "...adapter(farmfe_plugin_react_router_339bb3b2)"
      }
    ],
    "path": "",
    "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/_auth.lazy.tsx').then(adapter))"
  },
  {
    "path": "notes",
    "component": "lazy(() => import('/Users/cherry7/Documents/open/farm-fe/plugins/packages/react-router/playground/routes/notes.lazy.tsx').then(adapter))"
  }
];
