#! /usr/bin/bash

CONTAINER_NAME="hiddn-website-mysql-container"

start_container() {
    sudo docker run --name $CONTAINER_NAME \
        -e MYSQL_ROOT_PASSWORD=root \
        -e MYSQL_DATABASE=hiddn_website \
        -p 3306:3306 \
        -d mysql:latest
    echo "MySQL container started."
}

stop_container() {
    sudo docker stop $CONTAINER_NAME
    echo "MySQL container stopped."
}

restart_container() {
    sudo docker container restart $CONTAINER_NAME
    echo "MySQL container restarted."
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
restart)
    restart_container
    ;;
*)
    echo "Usage: $0 {start|stop|restart|remove}"
    exit 1
    ;;
esac
