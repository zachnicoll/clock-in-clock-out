#!/bin/bash

read_var() {
    VAR=$(grep $1 $2 | xargs)
    IFS="=" read -ra VAR <<< "$VAR"
    echo ${VAR[1]}
}

print() {
  echo -e "\e[32m$1 \e[0m \n"
}

POSTGRES_USER=$(read_var POSTGRES_USER docker.env)
POSTGRES_PASSWORD=$(read_var POSTGRES_PASSWORD docker.env)
POSTGRES_DB=$(read_var POSTGRES_DB docker.env)

print "[INFO] Shutting down docker containers..."
print "[CMD] sudo docker-compose down --remove-orphans"
sudo docker-compose down --remove-orphans

print "[INFO] Removing old dangling containers..."
print "[CMD] docker rmi --force $(sudo docker images -f "dangling=true" -q)"
sudo docker rmi --force $(sudo docker images -f "dangling=true" -q)

print "[INFO] Building production docker containers with:"
print "[CMD] sudo docker-compose -f docker-compose-prod.yml up -d --build--force-recreate"
sudo docker-compose -f docker-compose-prod.yml up -d --build --force-recreate

print "[INFO] Finished building containers \n"

cd ./backend

print "[INFO] Applying migrations with diesel: \n"
print "[CMD] diesel --database-url postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB} migration run"
diesel --database-url postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@localhost:5432/$POSTGRES_DB migration run

print "[INFO] Finished applying migrations! \n"

print "[INFO] Deployment complete - you may need to fix some errors if they occurred!"