### Build wasm bindings

```zsh
wasm-pack build ../peerdrop-frontend/crypto-wasm --target nodejs -d ../../peerdrop-backend/tests/pkg
```

## Running Script

```zsh
npx tsc
node dist/index.js
```
