int func(int a[][2], int i, int j) {
    return a[i][j];
}

int main() {
    int x[3][2];
    x[2][1] = 32;
    return func(x, 2, 1);
}
