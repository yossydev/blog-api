# Makefile

# Dockerイメージの名前
IMAGE_NAME=blog-api

# コンテナ名
CONTAINER_NAME=blog-api-container

# ビルドコマンド
build:
	docker build -t $(IMAGE_NAME) .

# 実行コマンド
run:
	docker run --name $(CONTAINER_NAME) -d -p 8080:8080 $(IMAGE_NAME)

# コンテナの停止と削除
clean:
	docker stop $(CONTAINER_NAME)
	docker rm $(CONTAINER_NAME)

# イメージの削除
clean-image:
	docker rmi $(IMAGE_NAME)

# コンテナとイメージの完全削除
clean-all: clean clean-image

.PHONY: build run clean clean-image clean-all
