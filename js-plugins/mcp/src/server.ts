import type { Compiler, Resource } from '@farmfe/core'
import type { FarmMcpOptions } from './types.js'
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js'
import { z } from 'zod'
import * as pkg from '../package.json'


export const createMcpDefaultServer = async (options: FarmMcpOptions, compiler: Compiler) => {

  const server = new McpServer(
    {
      name: 'farm',
      // @ts-ignore
      version: pkg.version,
      ...options.mcpServerInfo,
    },
  )

  server.tool("get-farm-config", "Get the Vite config digest, including the root, resolve, plugins", {}, async () => {

    const result = {
      root: compiler.config.config.root,
      resolve: compiler.config.config.resolve,
      plugins: compiler.config.jsPlugins.map(p => p.name).concat(compiler.config.rustPlugins.map(p => p[0])).filter(Boolean),
    }

    return {
      content: [{
        type: "text",
        text: JSON.stringify(result),
      }]
    }
  })


  server.tool("get-farm-resource-info", "Get graph information of a module, including importers, dynamicImports, importedBindings and exports,modules.", {
    filepath: z.string()
      .describe('The absolute filepath of the module'),
  }, async ({ filepath }) => {

    let resource = compiler.resourcesMap[filepath as keyof typeof compiler.resourcesMap] as Resource
    const result = {
      importers: resource.info?.data?.imports,
      dynamicImports: resource.info?.data?.dynamicImports,
      exports: resource.info?.data?.exports,
      importedBindings: resource.info?.data?.importedBindings,
      modules: resource.info?.modules,
    };

    return {
      content: [{
        type: "text",
        text: JSON.stringify(result),
      }]
    }
  })

  return server;
}