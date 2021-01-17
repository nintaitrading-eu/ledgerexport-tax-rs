# ledgerexport-tax
# See LICENSE file for copyright and license details.

include config.mk

all: ledgerexport-tax

ledgerexport-tax: ${OBJ}
	@echo ${CC} build
	@${CC} build

test:
	@echo running tests...
	@${CC} test

clean:
	@echo cleaning...
	@${CC} clean
	@rm -fv ledgerexport-tax-${VERSION}.tar.gz

dist: clean
	@echo creating dist tarball
	@mkdir -p ledgerexport-tax-${VERSION}
	@cp -R LICENSE.txt Makefile README.adoc config.mk \
		ledgerexport-tax.1 ${SRC} ledgerexport-tax-${VERSION}
	@tar -cf ledgerexport-tax-${VERSION}.tar ledgerexport-tax-${VERSION}
	@gzip ledgerexport-tax-${VERSION}.tar
	@rm -rf ledgerexport-tax-${VERSION}

install: all
	@echo installing executable file to ${DESTDIR}${PREFIX}/bin
	@mkdir -p ${DESTDIR}${PREFIX}/bin
	@cp -f ledgerexport-tax ${DESTDIR}${PREFIX}/bin
	@chmod 755 ${DESTDIR}${PREFIX}/bin/ledgerexport-tax
	@echo installing gnuplot scripts to ${DESTDIR}${PREFIX}/bin
	@mkdir -p ${DESTDIR}${SHARE}/ledgerexport-tax
	@cp -rfv gnuplot ${DESTDIR}${SHARE}/ledgerexport-tax/gnuplot
	@chmod 644 ${DESTDIR}${SHARE}/ledgerexport-tax/gnuplot/*
	@echo installing manual page to ${DESTDIR}${MANPREFIX}/man1
	@mkdir -p ${DESTDIR}${MANPREFIX}/man1
	@sed "s/VERSION/${VERSION}/g" < ledgerexport-tax.1 > ${DESTDIR}${MANPREFIX}/man1/ledgerexport-tax.1
	@chmod 644 ${DESTDIR}${MANPREFIX}/man1/ledgerexport-tax.1

uninstall:
	@echo removing executable file from ${DESTDIR}${PREFIX}/bin
	@rm -f ${DESTDIR}${PREFIX}/bin/ledgerexport-tax
	@echo removing data in /usr/local/share from ${DESTDIR}${SHARE}/ledgerexport-tax
	@rm -rf ${DESTDIR}${SHARE}/ledgerexport-tax
	@echo removing manual page from ${DESTDIR}${MANPREFIX}/man1
	@rm -f ${DESTDIR}${MANPREFIX}/man1/ledgerexport-tax.1

.PHONY: all options clean dist install uninstall
