CC ?= cc
CFLAGS ?= -std=c11 -O2 -Wall -Wextra -Werror -pedantic

all: inspect-deb
inspect-deb: inspect_deb.c
	$(CC) $(CFLAGS) -o $@ $<
clean:
	rm -f inspect-deb
.PHONY: all clean
