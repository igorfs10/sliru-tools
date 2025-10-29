# SlinRu Tools — Rust (desktop + WASM)

Pequena aplicação rust com interface gráfica slint com ferramentas simples para verificação de integridade de arquivos e de formatos de texto json,xml, yaml e csv.

## Pré-requisitos
- Rust estável (via `rustup`)
- **Web (WASM)**: `wasm-pack` (`cargo install wasm-pack`) e um servidor HTTP estático

---

## Executar (Desktop)
```bash
cargo run --bin sliru_tools
```
## Build (Desktop)
```bash
cargo build --release --bin sliru_tools
```

## Executar-build (WebAssembly)
```bash
# 1) Adicione o target wasm32
rustup target add wasm32-unknown-unknown

# 2) Gere os artefatos com wasm-pack
wasm-pack build --release --target web --out-dir web/pkg

# 3) Sirva a pasta web/ em um servidor estático
cd web
python -m http.server 5173
# Abra http://localhost:5173
```
> Dica: Você pode usar outro servidor (vite, serve, http-server, live-server, etc.).
