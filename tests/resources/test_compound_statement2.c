int main() {
    int a = 17;
    {
        a = 18;
        int a;
        a = 20;
    }
    return a;
}
