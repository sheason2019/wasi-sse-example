//@ts-check
import Koa from "koa";
import { PassThrough } from "stream";

async function main() {
  const app = new Koa();

  app
    .use(async (ctx, next) => {
      if (ctx.path !== "/sse") {
        return await next();
      }

      ctx.request.socket.setTimeout(0);
      ctx.req.socket.setNoDelay(true);
      ctx.req.socket.setKeepAlive(true);

      ctx.response.set({
        "Content-Type": "text/event-stream",
        "Cache-Control": "no-cache",
        Connection: "keep-alive",
      });

      const stream = new PassThrough();

      ctx.status = 200;
      ctx.body = stream;

      let i = 0;
      const interval = setInterval(() => {
        stream.write(`data: ${new Date()}\n\n`);
        if (i++ > 30) {
          clearInterval(interval);
        }
      }, 3000);
    })
    .use((ctx) => {
      ctx.status = 200;
      ctx.body = "ok";
    });

  app.listen(10000, () => console.log("Listening"));
}

main();
