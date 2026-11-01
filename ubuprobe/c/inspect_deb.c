/* Small educational Debian ar-format signature checker.
 * Does not parse or trust archive members, and does not install packages. */
#include <errno.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: inspect-deb FILE.deb\n");
        return 2;
    }
    FILE *file = fopen(argv[1], "rb");
    if (!file) {
        fprintf(stderr, "open failed: %s\n", strerror(errno));
        return 2;
    }
    unsigned char magic[8];
    size_t n = fread(magic, 1, sizeof magic, file);
    int read_error = ferror(file);
    if (fclose(file) != 0 || read_error) {
        fprintf(stderr, "read/close error\n");
        return 2;
    }
    if (n != sizeof magic || memcmp(magic, "!<arch>\n", 8) != 0) {
        fprintf(stderr, "not an ar-format archive\n");
        return 1;
    }
    puts("ar signature valid; Debian package structure NOT yet verified");
    return 0;
}
