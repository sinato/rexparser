int main() {
    int i = 0;
    int ans = 50;
    while (i < 10) {
        i += 1;
        if (i > 5) {
            continue;
        }
        ans += 1;
    }
    return ans;
}
