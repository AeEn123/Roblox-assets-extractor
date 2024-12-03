pkgname=Roblox-assets-extractor
pkgdesc='A safe way to extract assets from your Roblox installation.'
pkgver=0.1.3
pkgrel=1
makedepends=('rust' 'cargo')
arch=('x86_64')
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/AeEn123/Roblox-assets-extractor/archive/refs/tags/v0.1.3.tar.gz")
sha256sums=('1f8fbbf06c71266b81beb863e85de9feff2bdda0f3067397932f3c2c6f4d738f')
license=('MIT')

# Generated in accordance to https://wiki.archlinux.org/title/Rust_package_guidelines.
# Might require further modification depending on the package involved.
prepare() {
  mv ${srcdir}/${pkgname}-${pkgver}/* ${srcdir}/
  rm -rf ${srcdir}/${pkgname}-${pkgver}
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
  install -Dm644 "assets/icon.svg" "${pkgdir}/usr/share/icons/hicolor/symbolic/apps/Roblox-assets-extractor.svg"
  install -Dm644 /dev/stdin "${pkgdir}/usr/share/applications/Roblox-assets-extractor.desktop" <<EOF
  [Desktop Entry]
  Type=Application
  Name=Roblox Assets Extractor
  Comment=A safe way to extract assets from your Roblox installation.
  Categories=Utility
  Exec=Roblox-assets-extractor
  Icon=Roblox-assets-extractor
  Terminal=false
EOF
}
