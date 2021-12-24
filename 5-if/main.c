#include <stdio.h>

int main() {
    int age = 21;

    if (age >= 20) {
        printf("あなたはお酒が飲めます\n");
    } else if (age >= 0) {
        printf("あなたはお酒が飲めません\n");
    } else {
        printf("あなたはまだ生まれていません\n");
    }
    return 0;
}