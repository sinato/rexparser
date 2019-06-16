int main() {
    int val = 1;
    int ans = 0;
    if (val < 10) {
        if (val == 5) {
            ans += 10;
        } else {
            ans += 100;
        }
    } else {
        return 3;
    }
    return ans;
}
