
type TPreset = string | { from: string, imports: string[] } | {
  [key: string]: string | string[]
}

export interface IPluginOptions {
  dirs: string[];
  dts: boolean | string;
  ignore: string[];
  include: string[];
  exclude: string[];
  presets: TPreset[];
}
