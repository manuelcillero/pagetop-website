#!/bin/bash

cd "$(dirname "$0")"

cd en
mdbook build

cd ../es
mdbook build

