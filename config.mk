# ledgerplot version
VERSION = 0.0.1

# Customize below to fit your system

# paths
PREFIX = /usr/local
# Note: the below will be /share/man on linux.
MANPREFIX = ${PREFIX}/man
INCLUDE = /usr/src/include
SHARE = ${PREFIX}/share

# includes and libs
INCS = -I${INCLUDE}
LIBS =

# flags
# CPPFLAGS = -D_BSD_SOURCE -D_POSIX_C_SOURCE=2
# release
# CFLAGS   = -std=c11 -Wpedantic -Wall -Wno-deprecated-declarations -Os ${INCS} ${CPPFLAGS}
# debug
# CFLAGS = -g -c -Wall -Werror -std=c11 -O2 ${INCS}

# compiler and linker
CC = cargo -v
