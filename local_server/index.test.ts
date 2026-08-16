import { expect, test } from "bun:test";
import app from "./index";

test("トップページは plain HTML を返す", async () => {
  const response = await app.request("/");

  expect(response.status).toBe(200);
  expect(response.headers.get("content-type")).toContain("text/html");
  expect(await response.text()).toContain("<h1>saba local server</h1>");
});

test("ヘルスチェックは JSON を返す", async () => {
  const response = await app.request("/health");

  expect(response.status).toBe(200);
  expect(await response.json()).toEqual({ status: "ok" });
});
