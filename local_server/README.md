# local_server

`local_server` は、Bun と Hono で `index.html` を配信するローカル用の簡易サーバーです。

## 起動

`local_server` ディレクトリで依存関係をインストールし、開発サーバーを起動します。

```bash
bun install
bun run dev
```

ブラウザーで [http://localhost:3000](http://localhost:3000) を開くと、`index.html` が表示されます。

通常起動する場合は、次のコマンドを使います。

```bash
bun run start
```

ポートとホスト名は環境変数で変更できます。

```bash
PORT=4000 HOST=0.0.0.0 bun run start
```

## 確認

```bash
bun run test
bun run typecheck
```

`/health` にアクセスすると、サーバーの稼働状態を JSON で確認できます。
