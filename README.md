# geo-web-lib

## Developing

```sh
wasm-pack build

# or to rerun with changes in src/
fswatch -o src --event Updated | xargs -I{} wasm-pack build 
```

## Testing

```sh
wasm-pack test --headless --firefox
```

## Publishing

```
wasm-pack publish
```
