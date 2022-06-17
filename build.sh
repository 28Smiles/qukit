# /bin/sh

rm -rf dist
rm -rf pkg.*
mkdir dist

rm -rf pkg
wasm-pack build --target bundler -- --features wasm-pack
mv pkg pkg.bundler
rm pkg.bundler/{package.json,README.md,.gitignore}

rm -rf pkg
wasm-pack build --target nodejs -- --features wasm-pack
mv pkg pkg.node
rm pkg.node/{package.json,README.md,.gitignore}

rm -rf pkg
wasm-pack build --target web -- --features wasm-pack
mv pkg pkg.web
rm pkg.web/{package.json,README.md,.gitignore}

rm -rf pkg
wasm-pack build --target no-modules -- --features wasm-pack
mv pkg pkg.dist
rm pkg.dist/{package.json,README.md,.gitignore}
rm -rf pkg
