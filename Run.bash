#!/bin/bash

source ./ReTranslator/.env

kill $(lsof -t -i:[$SERVER_PORT])

if [[ "$OSTYPE" == "darwin"* ]]; then
    bash ./bash/RunMac.bash
fi