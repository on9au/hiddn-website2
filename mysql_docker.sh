#! /usr/bin/bash

CONTAINER_NAME="hiddn-website-mysql-container"

start_container() {
    sudo docker run --name $CONTAINER_NAME \
        -e MYSQL_ROOT_PASSWORD=root \
        -e MYSQL_DATABASE=devdb \
        -p 3306:3306 \
        -d mysql:latest
    echo "MySQL container started."
}

stop_container() {
    sudo docker stop $CONTAINER_NAME
    echo "MySQL container stopped."
}

remove_container() {
    sudo docker rm $CONTAINER_NAME
    echo "MySQL container removed."
}

case "$1" in
    start)
        start_container
        ;;
    stop)
        stop_container
        ;;
    remove)
        remove_container
        ;;
    *)
        echo "Usage: $0 {start|stop|remove}"
        exit 1
        ;;
esac