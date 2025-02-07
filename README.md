# rust-api

このプロジェクトは、Rust を用いた API サーバーのサンプル実装です。

## ディレクトリ構成

```
.
├── README.md
├── backend
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
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

### 1. Rust の依存関係をインストールする

まず、`backend` ディレクトリに移動し、以下のコマンドを実行してください。

```sh
cd backend
cargo build
```

### 2. Docker コンテナをビルドして起動する

```sh
docker-compose up --build
```

## 各種コマンド

backend ディレクトリ直下で、以下のコマンドが利用できます。

```sh
cargo watch -x run
cargo run
```

## API エンドポイント

```sh
curl http://localhost:8080/tasks # タスク一覧の取得
curl http://localhost:8080/tasks/1 # 特定タスクの取得 (例: ID = 1)
curl -X POST http://localhost:8080/tasks # タスクの作成
curl -X PUT http://localhost:8080/tasks/1 # タスクの更新 (例: ID = 1)
curl -X DELETE http://localhost:8080/tasks/1 # タスクの削除 (例: ID = 1)
```
