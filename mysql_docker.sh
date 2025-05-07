#!/usr/bin/env bash

CONTAINER_NAME="hiddn-website-mysql-container"
IMAGE_NAME="mysql:latest"
MYSQL_ROOT_PASSWORD="rootpassword"
MYSQL_DATABASE="hiddn_db"

case "$1" in
start)
    # Check if container exists
    if sudo docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
        echo "🔄 Starting existing MySQL container..."
        sudo docker start "$CONTAINER_NAME"
    else
        echo "🐳 Creating and starting new MySQL container..."
        sudo docker run -d \
            --name "$CONTAINER_NAME" \
            -e MYSQL_ROOT_PASSWORD="$MYSQL_ROOT_PASSWORD" \
            -e MYSQL_DATABASE="$MYSQL_DATABASE" \
            -p 3306:3306 \
            "$IMAGE_NAME"
    fi
    ;;
stop)
    echo "🛑 Stopping MySQL container..."
    sudo docker stop "$CONTAINER_NAME"
    ;;
remove)
    echo "❌ Removing MySQL container..."
    sudo docker rm -f "$CONTAINER_NAME"
    ;;
*)
    echo "Usage: $0 {start|stop|remove}"
    ;;
esac
