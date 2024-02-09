PROJECT_NAME := blog-api
SERVICE_NAME := blog-api

# コンテナをビルドするコマンド
build:
	docker-compose build $(SERVICE_NAME)

# コンテナをバックグラウンドで起動するコマンド
up:
	docker-compose up -d $(SERVICE_NAME)

# コンテナを停止し削除するコマンド
down:
	docker-compose down

# コンテナとそのボリュームを停止し削除するコマンド
down-volumes:
	docker-compose down -v

# コンテナのログを表示するコマンド
logs:
	docker-compose logs -f $(SERVICE_NAME)

# 不要なDockerオブジェクトをクリーンアップするコマンド
clean:
	docker system prune -af
	docker volume prune -f

# 任意のコマンドをコンテナ内で実行する
exec:
	docker-compose exec $(SERVICE_NAME) /bin/sh

# コンテナを再ビルドし、再起動するコマンド
rebuild: down build up

.PHONY: build up down down-volumes logs clean exec rebuild
