#! /usr/bin/bash

sudo docker run --name hiddn-website-mysql-container \
    -e MYSQL_ROOT_PASSWORD=root \
    -e MYSQL_DATABASE=devdb \
    -p 3306:3306 \
    -d mysql:latest