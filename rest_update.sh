#!/bin/bash

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"ad":{"id":"123","title":"T1","description":"D1","lock":"5645"}}' \
    http://localhost:8080/v1/update

    