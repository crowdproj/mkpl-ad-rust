#!/bin/bash

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"admin"}' \
    http://localhost:8080/v1/create

    