#!/bin/bash

curl -X POST -vv\
    -H "Content-Type: application/json" \
    -d '{"ad":{"id":"T1"}}' \
    http://localhost:8080/v1/offers

    