#!/bin/sh

printf 'P2
2 2
255
1 2
3 4
' |
    wasmtime run ./rs-img2pnm.wasm --max-input-bytes 1024 |
    wasmtime run ./rs-img2pnm.wasm |
    file -
