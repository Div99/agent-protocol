#!/bin/bash

mkdir tmp
cd tmp

openapi-generator-cli generate -i ../../schemas/openapi.yml -g rust

mv ./src/apis ../src/
mv ./src/models ../src/
mv ./src/lib.rs ../src/
mv ./docs ../docs

cd ../

rm -rfv tmp
