#!/bin/bash

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"filter":{"search_string":""}}' \
    http://localhost:8080/v1/search

    