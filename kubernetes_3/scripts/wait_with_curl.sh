#!/usr/bin/env bash

set -eu

echo "Waiting for http://arch.homework to get up & running"
until $(curl --output /dev/null --silent --head --fail http://arch.homework/health); do
    printf '.'
    sleep 1
done
echo "Done"
