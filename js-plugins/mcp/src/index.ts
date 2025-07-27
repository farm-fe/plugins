import type { JsPlugin, ResolvedUserConfig } from '@farmfe/core';
import { FarmMcpOptions } from './types.js';
import { setupRoutes } from './connect.js';
import { createMcpDefaultServer } from "./server.js"
import { searchForWorkspaceRoot } from "./search-root.js"
import p from "picocolors"
import fs from 'fs/promises';
import { existsSync } from 'fs';
import { join } from 'path';


export default function farmPluginMcp(options: FarmMcpOptions): JsPlugin {
  const {
    mcpPath = '/__mcp',
    updateCursorMcpJson = true,
    updateVSCodeMcpJson = true,
    printUrl = true,
  } = options
  const cursorMcpOptions = typeof updateCursorMcpJson == 'boolean'
    ? { enabled: updateCursorMcpJson }
    : updateCursorMcpJson

  const vscodeMcpOptions = typeof updateVSCodeMcpJson == 'boolean'
    ? { enabled: updateVSCodeMcpJson }
    : updateVSCodeMcpJson
  let farmConfig: ResolvedUserConfig = {}
  return {
    name: 'mcp',
    configResolved(config) {
      config = config
    },
    async configureDevServer(server) {
      const compiler = server.getCompiler();
      let mcp = await createMcpDefaultServer(options, compiler);
      mcp = await options.mcpServerSetup?.(mcp, compiler) || mcp;
      setupRoutes(mcpPath || '/__mcp', mcp, server);

      // const config = 
      const port = server.config.port;
      const protocol = server.config.https ? 'https' : 'http'
      const sseUrl = `${protocol}://${options.host || 'localhost'}:${options.port || port}${mcpPath}/sse`


      const root = searchForWorkspaceRoot(farmConfig.root ?? process.cwd());
      if (cursorMcpOptions.enabled) {
        if (existsSync(join(root, '.cursor'))) {
          const mcp = existsSync(join(root, '.cursor/mcp.json'))
            ? JSON.parse(await fs.readFile(join(root, '.cursor/mcp.json'), 'utf-8') || '{}')
            : {}
          mcp.mcpServers ||= {}
          mcp.mcpServers[cursorMcpOptions.serverName || 'farm'] = { url: sseUrl }
          await fs.writeFile(join(root, '.cursor/mcp.json'), `${JSON.stringify(mcp, null, 2)}\n`)
        }
      }

      if (vscodeMcpOptions.enabled) {
        const vscodeConfig = join(root, '.vscode/settings.json')
        if (existsSync(vscodeConfig)) {
          const mcp = existsSync(join(root, '.vscode/mcp.json'))
            ? JSON.parse(await fs.readFile(join(root, '.vscode/mcp.json'), 'utf-8') || '{}')
            : {}
          mcp.servers ||= {}
          mcp.servers[vscodeMcpOptions.serverName || 'farm'] = {
            type: 'sse',
            url: sseUrl,
          }
          await fs.writeFile(join(root, '.vscode/mcp.json'), `${JSON.stringify(mcp, null, 2)}\n`)
        }
      }

      if (printUrl) {
        setTimeout(() => {
          console.log(`${p.yellow(`  ➜  MCP: Server is running at ${sseUrl}`)}`)
        }, 300)
      }

    },
  };
}
