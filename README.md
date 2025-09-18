# rust-api

このプロジェクトは、Rust を用いた API サーバーのサンプル実装です。
hoge

## ディレクトリ構成

```
.
├── README.md
├── backend
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── db
│   └── init.sql
└── docker-compose.yml
```

## 前提条件

このプロジェクトをセットアップする前に、下記のツールがローカルマシンにインストールされている必要があります。

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker Compose](https://docs.docker.com/compose/install/)

## セットアップ手順

### 1. Docker コンテナをビルドし、起動する

```sh
docker-compose up --build
```

### 2. Rust を起動する

まず、`backend` ディレクトリに移動し、以下のコマンドを実行してください。

```sh
cd backend
cargo run
```

## API エンドポイント

```sh
curl http://localhost:8080/tasks # タスク一覧の取得
curl http://localhost:8080/tasks/1 # 特定タスクの取得 (例: ID = 1)
curl -X POST -H "Content-Type: application/json" \
  -d '{"description": "新しいタスク", "is_completed": false}' \
  http://localhost:8080/tasks # タスクの作成
curl -X PUT -H "Content-Type: application/json" \
  -d '{"description": "更新されたタスク", "is_completed": true}' \
  http://localhost:8080/tasks/1 # タスクの更新 (例: ID = 1)
curl -X DELETE http://localhost:8080/tasks/1 # タスクの削除 (例: ID = 1)
```
