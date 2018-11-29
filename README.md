My playground for: https://github.com/rustwasm/wasm-bindgen/issues/1046

To test the above issue failure:

```
wasm-pack test --headless --firefox
```

To rebuild pkg:

```
wasm-pack build --target no-modules
```

Run your favorite development web server, e.g.:

```
devd -o pkg/
```
