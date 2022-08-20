# This is a justfile (https://github.com/casey/just)

# render project
build-and-sign:
  cargo build --target=aarch64-apple-darwin --release && \
  cargo build --target=x86_64-apple-darwin --release && \
  lipo -create -output "${HOME}/bin/ohq2quarto" target/aarch64-apple-darwin/release/ohq2quarto target/x86_64-apple-darwin/release/ohq2quarto && \
  codesign --force --verify --verbose --sign "${APPLE_SIGN}" "${HOME}/bin/ohq2quarto"

# the following only works on my linux box b/c "Apple Silicon"
build-x86_64-pc-windows-gnu: 
  cross build --target x86_64-pc-windows-gnu

build-aarch64-unknown-linux-gnu:
 cross build --target aarch64-unknown-linux-gnu

build-i686-pc-windows-gnu:
  cross build --target i686-pc-windows-gnu

build-i686-pc-windows-msvc:
  cross build --target i686-pc-windows-msvc

build-i686-unknown-linux-gnu:
  cross build --target i686-unknown-linux-gnu

build-x86_64-apple-darwin:
  cross build --target x86_64-apple-darwin

build-x86_64-pc-windows-gnu:
  cross build --target x86_64-pc-windows-gnu

build-x86_64-pc-windows-msvc:
  cross build --target x86_64-pc-windows-msvc

build-x86_64-unknown-linux-gnu:
  cross build --target x86_64-unknown-linux-gnu
