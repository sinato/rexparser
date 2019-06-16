int func(int val) {
    if (val == 7) {
        return 11;
    } else {
        return 80;
    }
}

int main() {
    return func(7) + func(0);
}
