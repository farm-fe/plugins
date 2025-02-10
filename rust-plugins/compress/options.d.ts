export interface IPluginOptions {
	algorithm?: "gzip" | "brotli" | "deflateRaw";
	filter?: string;
	deleteOriginFile?: boolean;
}
