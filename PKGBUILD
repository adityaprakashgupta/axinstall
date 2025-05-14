pkgname=axinstall-cli
pkgver=1.7
pkgrel=1
pkgdesc="AxOS installer backend"
arch=('x86_64')
license=('GPL')
makedepends=('cargo' 'rust')
depends=('arch-install-scripts') # this is needed because it contains arch-chroot
# sha256sums=('SKIP') 

build() {
  cd "${srcdir}"
  cargo build --release --locked
}


package() {
  cd "${srcdir}/"
  install -Dm755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}
