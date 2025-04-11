import type { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js'
import type { Server } from '@farmfe/core'
import { SSEServerTransport } from '@modelcontextprotocol/sdk/server/sse.js'
import { Context, Next } from "koa"


export async function setupRoutes(base: string, mcpServer: McpServer, farmServer: Server): Promise<void> {
  const transports = new Map<string, SSEServerTransport>()

  farmServer.applyMiddlewares([() => {

    return async (ctx: Context, next: Next) => {
      if (!ctx.url.includes(`${base}/sse`)) {
        return next()
      }
      const transport = new SSEServerTransport(`${base}/messages`, ctx.res);
      transports.set(transport.sessionId, transport);
      ctx.res.on('close', () => {
        transports.delete(transport.sessionId);
      });
      await mcpServer.connect(transport)
    }

  }])

  farmServer.applyMiddlewares([() => {
    return async (ctx: Context, next: Next) => {
      if (!ctx.url?.includes(`${base}/messages`)) {
        return next()
      }
      if (ctx.method !== 'POST') {
        ctx.status = 405
        ctx.body = 'Method Not Allowed'
        return
      }

      const query = new URLSearchParams(ctx.url?.split('?').pop() || '')
      const clientId = query.get('sessionId')

      if (!clientId || typeof clientId !== 'string') {
        ctx.status = 400
        ctx.body = 'Bad Request'
        return
      }

      const transport = transports.get(clientId)
      if (!transport) {
        ctx.status = 404
        ctx.body = 'Not Found'
        return
      }

      await transport.handlePostMessage(ctx.req, ctx.res)
      next()
    }
  }])

}