#!/bin/bash

full_version=$(cargo run -- -V)
version_number=$(echo "$full_version" | awk '{print $2}')

touch spin.adoc

asciidoctor -b manpage spin.adoc \
  --destination=../man/ \
  --attribute release-version="$version_number"
