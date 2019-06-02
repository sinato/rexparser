int main() {
    int a[3];
    a[1] = 22;
    int *a_p;
    a_p = a + 1;
    return *a_p;
}
