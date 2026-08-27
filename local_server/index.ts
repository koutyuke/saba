import { Hono } from "hono";
import { serveStatic } from "hono/bun";

const app = new Hono();
const indexFile = Bun.file(`${import.meta.dir}/index.html`);

app.get("/", async (c) => c.html(await indexFile.text()));

app.get("/health", (c) => c.json({ status: "ok" }));

app.get("/*", serveStatic({ root: `${import.meta.dir}/public` }));

if (import.meta.main) {
    const server = Bun.serve({
        hostname: Bun.env.HOST ?? "127.0.0.1",
        port: Number(Bun.env.PORT ?? 8000),
        fetch: app.fetch,
    });

    console.log(`local_server is listening on ${server.url}`);
}

export default app;
