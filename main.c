#include <stdio.h>
#include <stdint.h>
#include <unistd.h>

void dump(uint64_t x)
{
    char buf[32];
    size_t buf_sz = 0;
    do {
        buf[sizeof(buf) - buf_sz - 1] = x % 10 + '0';
        buf_sz++;
        x /= 10;
    } while(x);
    write(1, &buf[sizeof(buf) - buf_sz], buf_sz);
    printf("\n");
}

int main()
{
    dump(69);
    dump(420);
    return 0;
}
