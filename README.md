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
│       └── main.rs
└── docker-compose.yml
```

## セットアップ

### 前提

下記がローカルマシンにインストールされていること。

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

## 動作チェック

```sh
curl http://localhost:8080/
```
