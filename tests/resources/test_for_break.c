int main() {
    int ans = 0;
    for (int i = 0; i < 10; i++) {
        ans += i;
        if (i == 5) {
            break;
        }
    }
    return ans;
}
