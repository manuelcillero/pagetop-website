#!/bin/bash

cd "$(dirname "$0")"

cd v0.0
cd en ; mdbook build
cd ..
cd es ; mdbook build
