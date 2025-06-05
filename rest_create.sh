#!/bin/bash

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"ad":{"title":"T1","description":"D1"}}' \
    http://localhost:8080/v1/create

    