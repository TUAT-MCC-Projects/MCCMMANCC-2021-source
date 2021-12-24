#include <stdio.h>

int main() {
    for (int i = 0; i < 10; i++) {
        printf("%d\n", i);
    }

    int s = 0;
    while (s < 20) {
        s++;
    }
    printf("%d\n", s);

    // while (1) {
    //     printf("無限ループ");
    // }
}