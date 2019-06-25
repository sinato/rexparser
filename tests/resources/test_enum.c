int main() {
    enum Color {
        RED,
        BLUE,
        GREEN = 6,
        BLACK,
    };

    enum Color c;
    c = BLUE;

    int a = BLACK;
    return c + a * 10;
}
