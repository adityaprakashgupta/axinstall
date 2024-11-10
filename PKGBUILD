pkgname=axinstall-cli
pkgver=0.1.13
pkgrel=2
pkgdesc="AxOS installer backend"
arch=('x86_64')
license=('GPL')
makedepends=('cargo' 'rust')
# sha256sums=('SKIP') 

build() {
  cd "${srcdir}"
  cargo build --release --locked
}


package() {
  cd "${srcdir}/"
  install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
