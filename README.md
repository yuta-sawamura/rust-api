# rust-api

## ディレクトリ構成

```
.
├── README.md
├── backend
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src
│       └── main.rs         // アプリケーションエントリーポイント。サーバ起動・ミドルウェア設定等
|       ├── config.rs       // 設定情報（例：環境変数やポート番号等）の定義
|       ├── db.rs           // DB接続やコネクションプールの初期化
|       ├── models          // データ構造の定義
|       │   └── task.rs     // Todoタスクのモデル定義（例：struct Task）
|       ├── handlers        // 各CRUD操作のハンドラ関数
|       │   └── task.rs     // タスクに関する処理（一覧取得、作成、更新、削除など）
|       └── routes          // ルーティングの定義
|           └── task.rs     // エンドポイントと対応するハンドラのマッピング
├── db
│   └── init.sql
└── docker-compose.yml
```

## セットアップ

前提として、下記がローカルマシンにインストールされていること。

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker Compose](https://docs.docker.com/compose/install/)

1. Rust の依存関係をインストールする

```sh
cd backend
cargo build
```

2. Docker コンテナをビルドして起動する

```sh
docker-compose up --build
```

## 各コマンド

```sh
curl http://localhost:8080/ # 動作チェック
# backend直下
cargo watch -x run
cargo run
```
