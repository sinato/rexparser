int func(int a) {
    if (a == 10) {
        return 77;
    }
    return 22;
}

int main() {
    return func(10) + func(0);
}
