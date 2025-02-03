FROM rust:latest

# 開発向けにcargo-watchをインストール
RUN cargo install cargo-watch

WORKDIR /app

# ポート公開(Actixのデフォルトで8080など)
EXPOSE 8080

# コンテナ起動時に cargo-watch を使って、ソース変更を検知しながら run
CMD ["cargo", "watch", "-x", "run"]
