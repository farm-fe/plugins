import { defineConfig } from "@farmfe/core";
import worker from "@farmfe/plugin-worker";

export default defineConfig({
	plugins: [worker(), "@farmfe/plugin-react"],
	compilation: {
		output: {
			assetsFilename: "public/[resourceName].[ext]", // [] 里面的是 Farm 支持的全部占位符
		},
		assets: {
			include: ["py"], // 额外静态资源类型
		},
	},
});
