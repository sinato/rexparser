int main() {
    int a[2][3];
    a[0][0] = 1;
    a[0][1] = 2;
    a[0][2] = 3;
    a[1][0] = 10;
    a[1][1] = 20;
    a[1][2] = 30;
    return a[0][0] + a[0][1] + a[0][2] + a[1][0] + a[1][1] + a[1][2];
}
