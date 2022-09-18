#!/bin/bash

docker run --rm --name test-postgres -p 5432:5432 -e POSTGRES_PASSWORD=localdb -d postgres    