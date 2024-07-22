import { defineConfig } from "@farmfe/core";
import farmJsPluginPostcss from '@farmfe/js-plugin-postcss';
import visualizer from '@farmfe/js-plugin-visualizer'
export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    farmJsPluginPostcss(),
    visualizer(),
    ["@farmfe/plugin-react", { runtime: "automatic" }],
    ["@farmfe/plugin-icons", {
      /**
       * zie of zooming icon
       * @type {float}
       * @default 1.2
       */
      scale: 1,
      /**
       * @description Whether to automatically install the required dependencies
       * @type {boolean}
       * @default true
       */
      autoInstall: true,
      /**
       * @description The compiler used by the plugin
       * @type {string}
       * @default "jsx"
       * @enum ["jsx", "vue","react","preact","solid","svelte"]
       */
      compiler: "jsx",
      /**
       * @description The default style to apply to the svg element
       * @type {object}
       * @default {}
       */
      defaultStyle: {},
      /**
       * @description The default class to apply to the svg element
       * @type {string}
       */
      defaultClass: "",
      /**
       * @description Custom icon collection, support local svg and remote svg
       * @type {string}
       * @uses [iconname] to replace the icon name
       * @ex
       *  import icon from "~icons/local/icon.svg"
       *  import icon from "~icons/remote/icon.svg"
       */
      customCollections: {
        local: './src/assets',
        remote: "https://cdn.simpleicons.org/[iconname]/"
      }
    }],
  ],
});
