import { afterAll, beforeAll, expect, test } from "bun:test";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { basename, dirname, join } from "node:path";
import app from "./index";

const fixtures = [
  { path: "sample.html", body: "<h1>Static page</h1>", type: "text/html" },
  { path: "nested/style.css", body: "h1 { color: red; }", type: "text/css" },
  { path: "日本語.txt", body: "日本語のファイル", type: "text/plain" },
];
let fixtureDir: string;

beforeAll(async () => {
  const publicDir = join(import.meta.dir, "public");
  await mkdir(publicDir, { recursive: true });
  fixtureDir = await mkdtemp(join(publicDir, ".static-test-"));

  for (const fixture of fixtures) {
    const path = join(fixtureDir, fixture.path);
    await mkdir(dirname(path), { recursive: true });
    await writeFile(path, fixture.body);
  }
});

afterAll(async () => {
  if (fixtureDir) {
    await rm(fixtureDir, { recursive: true, force: true });
  }
});

test("トップページは plain HTML を返す", async () => {
  const response = await app.request("/");

  expect(response.status).toBe(200);
  expect(response.headers.get("content-type")).toContain("text/html");
  expect(await response.text()).toBe(
    await Bun.file(`${import.meta.dir}/index.html`).text(),
  );
});

test.each(fixtures)("public 内の $path を返す", async (fixture) => {
  const path = encodeURI(`/${basename(fixtureDir)}/${fixture.path}`);
  const response = await app.request(`${path}?v=1`);

  expect(response.status).toBe(200);
  expect(response.headers.get("content-type")).toContain(fixture.type);
  expect(await response.text()).toBe(fixture.body);
});

test.each([
  "/missing-file.html",
  "/index.ts",
  "/package.json",
  "/..%2fpackage.json",
  "/%2e%2e%5cpackage.json",
])("存在しないファイルや公開対象外の %s は 404 を返す", async (path) => {
  const response = await app.request(path);

  expect(response.status).toBe(404);
});

test("ヘルスチェックは JSON を返す", async () => {
  const response = await app.request("/health");

  expect(response.status).toBe(200);
  expect(await response.json()).toEqual({ status: "ok" });
});
