# local_server

`local_server` は、Bun と Hono で HTML や静的ファイルを配信するローカル用の簡易サーバーです。

- `/` は、このディレクトリの `index.html` を返します。
- それ以外のパスは、`public/` 内の同じ名前のファイルを返します。たとえば `/test1.html` は `public/test1.html`、`/test2.html` は `public/test2.html` に対応します。
- `/health` は例外で、稼働状態を JSON で返します。
- 対応するファイルがない場合は `404 Not Found` を返します。

配信したいファイルは `public/` に追加してください。CSS や画像、サブディレクトリ内のファイルも同じルールで配信できます。

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
