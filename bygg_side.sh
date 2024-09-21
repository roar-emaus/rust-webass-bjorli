#!/usr/bin/bash

pushd bjorli
wasm-pack build --target web
popd

rm -r side
mkdir side

cp -r bjorli/index.html bjorli/pkg bjorli/style.css side/
rm side/pkg/.gitignore


