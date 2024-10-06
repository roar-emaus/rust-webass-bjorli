#!/usr/bin/bash


# Build with wasm-pack
pushd bjorli
wasm-pack build --target web
popd

# Copy files to the 'side' directory
rm -r side
mkdir side
cp -r bjorli/index.html bjorli/pkg bjorli/style.css bjorli/bever.css side/
rm side/pkg/.gitignore

# File containing the version number
HTML_FILE="side/index.html"

# Extract the current version number from the HTML file
VERSION=$(grep -o 'version=[0-9]*' $HTML_FILE | grep -o '[0-9]*')

# Check if version exists, if not, set to 0
if [ -z "$VERSION" ]; then
    VERSION=0
fi

# Increment the version number
NEW_VERSION=$((VERSION + 1))

# Update the version number in the HTML file
sed -i "s/version=$VERSION/version=$NEW_VERSION/" $HTML_FILE

echo "Build version: $NEW_VERSION"
