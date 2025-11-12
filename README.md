# SlinRu Tools — Rust (desktop + WASM)

Pequena aplicação rust com interface gráfica slint com ferramentas simples para verificação de integridade de arquivos e de formatos de texto json,xml, yaml e csv.
Versão web: https://igorfs10.github.io/sliru-tools/web/

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
# 1) Instalar wasm-pack (se ainda não tiver)
cargo install wasm-pack

# 2) Adicione o target wasm32
rustup target add wasm32-unknown-unknown

# 3) Gere os artefatos com wasm-pack
#Windows
$env:RUSTFLAGS="--cfg=web_sys_unstable_apis"
wasm-pack build --release --target web --out-dir web/pkg
#linux
export RUSTFLAGS="--cfg=web_sys_unstable_apis"
wasm-pack build --release --target web --out-dir web/pkg

# 4) Sirva a pasta web/ em um servidor estático
cd web
python -m http.server 5173
# Abra http://localhost:5173
```
> Dica: Você pode usar outro servidor (vite, serve, http-server, live-server, etc.).

## Gerar .pot tradução

```bash
# 1) Instale o slint-tr-extractor (se ainda não tiver)
cargo install slint-tr-extractor

# 2) Gere o arquivo .pot
cd ui
find -name \*.slint | xargs slint-tr-extractor -o sliru-tools.pot

# 3) Mova o arquivo .pot para a pasta correta
mv sliru-tools.pot ../translations/
```

## HDoc request format

```
<<METHOD
GET
METHOD
<<URL
https://igorfs10.github.io/PokemonSite/api/1/
URL
<<HEADERS
Content-Type: application/json
HEADERS
<<BODY
{
	"name": "name",
	"phone": "1234532"
}
BODY
```
> METHOD and URL are required