int func(int a[3]) {
    return a[1];
}

int main() {
    int x[3];
    x[1] = 35;
    return func(x);
}
