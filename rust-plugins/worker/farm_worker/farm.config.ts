import { defineConfig } from "@farmfe/core";
import worker from "@farmfe/plugin-worker";

export default defineConfig({
	plugins: [worker(), "@farmfe/plugin-react"],
  server:{
    port: 3000,
  },
	compilation: {
		output: {
			assetsFilename: "public/[resourceName].[ext]", // [] 里面的是 Farm 支持的全部占位符
		},
    persistentCache: false,
		assets: {
			include: ["py"], // 额外静态资源类型
		},
	},
});
