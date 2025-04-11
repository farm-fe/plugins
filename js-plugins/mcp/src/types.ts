import type { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js'
import type { Implementation as McpServerInfo } from '@modelcontextprotocol/sdk/types.js'
import { Compiler, type Server } from "@farmfe/core"

export type { McpServer }

type Awaitable<T> = T | PromiseLike<T>;


export interface FarmMcpOptions {
  /**
   * The host to listen on, default is `localhost`
   */
  host?: string

  /**
   * The port to listen on, default is the port of the Vite dev server
   */
  port?: number

  /**
   * Print the MCP server URL in the console
   *
   * @default true
   */
  printUrl?: boolean

  /**
   * The MCP server info. Ingored when `mcpServer` is provided
   */
  mcpServerInfo?: McpServerInfo

  /**
   * Setup the MCP server, this is called when the MCP server is created
   * You may also return a new MCP server to replace the default one
   */
  mcpServerSetup?: (server: McpServer, farmServer: Compiler) => Awaitable<void | McpServer>

  /**
   * The path to the MCP server, default is `/__mcp`
   */
  mcpPath?: string

  /**
   * Update the address of the MCP server in the cursor config file `.cursor/mcp.json`,
   * if `.cursor` folder exists.
   *
   * @default true
   */
  updateCursorMcpJson?: boolean | {
    enabled: boolean
    /**
     * The name of the MCP server, default is `vite`
     */
    serverName?: string
  }

  /**
   * Update the address of the MCP server in the VSCode config file `settings.json`,
   * if VSCode config file exists.
   *
   * @default true
   */
  updateVSCodeMcpJson?: boolean | {
    enabled: boolean
    /**
     * The name of the MCP server, default is `vite`
     */
    serverName?: string
  }
}