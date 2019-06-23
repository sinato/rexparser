int func(int a[3]) {
    return a[1];
}

int func1(int a[]) {
    return a[0];
}

int main() {
    int x[3];
    x[0] = 10;
    x[1] = 25;
    return func(x) + func1(x);
}
