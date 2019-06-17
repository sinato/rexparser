int func(int val1, int val2) {
    if(val1 == 1 || val2 == 2) {
        return val1 + val2;
    } else {
        return 0;
    }
}

int main() {
    return func(0, 2) + func(1, 0) + func(10, 2) + func(0, 0);
}
