#!/bin/bash
# This script copies static files to the dist folder to node_modules

HTMX_VERSION=$(jq -r '.dependencies."htmx.org"' package.json)
IDIOMORPH_VERSION=$(jq -r '.dependencies."idiomorph"' package.json)

# Check if node_modules folder exists
if [ ! -d "node_modules" ]; then
  echo "node_modules folder does not exist. Please run npm install first."
  exit 1
fi

# Clear folder if dist folder exists
if [ -d "dist" ]; then
  read -p "Removing static/dist folder, are you sure? (y/n): " -n 1 -r
  echo
  if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -rf dist
  else
    echo "Alrighty then, exiting."
    exit 1
  fi
fi
mkdir dist

cp -r ./node_modules/htmx.org/dist/htmx.js "./dist/htmx-$HTMX_VERSION.js"
cp -r ./node_modules/htmx.org/dist/ext/response-targets.js "./dist/htmx-response-targets-$HTMX_VERSION.min.js"
cp -r ./node_modules/idiomorph/dist/idiomorph-ext.min.js "./dist/idiomorph-$IDIOMORPH_VERSION.min.js"

echo "Done, static files copied to dist folder."
