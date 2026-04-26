# github_pages

Rust + Dioxus + WebAssembly で作ったパーソナルサイト。GitHub Pages にデプロイされます。

## 概要

| 項目 | 内容 |
|------|------|
| フレームワーク | [Dioxus](https://dioxuslabs.com/) 0.6 (WASM) |
| スタイリング | Tailwind CSS v3 |
| ビルドターゲット | `wasm32-unknown-unknown` |
| デプロイ | GitHub Actions → GitHub Pages |

### ページ構成

| URL | 内容 |
|-----|------|
| `/` | ホーム - 黒猫 を動かすインタラクティブな画面 |
| `/blog` | ブログ記事一覧 |
| `/blog/:id` | ブログ記事詳細 (Markdown) |

### 機能

- 矢印キーまたは画面上のコントローラーで猫キャラ (黒猫) を移動
- 猫をクリックすると RPG 風メッセージが1文字ずつ表示
- Web Audio API を使った BGM 再生 / 停止ボタン

## 必要な環境

- Rust (stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/)
- Node.js + npm (Tailwind CSS ビルド用)

```bash
# Dioxus CLI インストール
cargo install dioxus-cli

# WASM ターゲット追加
rustup target add wasm32-unknown-unknown
```

## 開発

ターミナルを2つ開いて、それぞれ以下を実行します。

### 1. Tailwind CSS ウォッチ

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### 2. 開発サーバー起動

```bash
dx serve
```

ブラウザで `http://localhost:8080` が開きます。ファイルを変更すると自動でリロードされます。

## プロダクションビルド

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --minify
dx build --release
```

ビルド成果物は `target/dx/github_pages/release/web/public/` に出力されます。

## ブログ記事を追加する

1. `assets/blogs/<番号>.md` に Markdown ファイルを追加
2. `assets/blogs/index.json` にエントリを追加

```json
{ "id": 6, "file": "6.md" }
```

3. `src/views/blog_list.rs` の `get_blog_md` 関数に `"6.md" => BLOG_6` を追記
4. ファイル先頭に `const BLOG_6: &str = include_str!("../../assets/blogs/6.md");` を追加

> Markdown の1行目の `# 見出し` が記事タイトルとして、最初の段落が一覧ページの抜粋として使われます。

## デプロイ

`main` ブランチにプッシュすると GitHub Actions が自動でビルドして `gh-pages` ブランチにデプロイします。

```
.github/workflows/gh-pages.yml
```

## プロジェクト構成

```
github_pages/
├── assets/
│   ├── blogs/          # Markdown ブログ記事 + index.json
│   ├── styling/        # CSS ファイル
│   └── favicon.ico
├── src/
│   ├── main.rs         # エントリポイント + ルーティング定義
│   ├── components/
│   │   ├── bgm.rs      # BGM (Web Audio API)
│   │   ├── cat.rs      # 猫キャラ SVG コンポーネント
│   │   ├── controller.rs  # 十字キー UI
│   │   └── jiji.rs     # ホーム画面メインコンポーネント
│   └── views/
│       ├── home.rs     # / ルート
│       ├── blog_list.rs   # /blog ルート
│       ├── blog.rs     # /blog/:id ルート
│       └── navbar.rs   # ナビゲーションバー
├── Cargo.toml
├── Dioxus.toml
└── tailwind.config.js
```
