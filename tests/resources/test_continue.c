int main() {
    int ans = 100;
    for(int i = 0; i < 5; i++) {
        if (i == 2) {
            continue;
        }
        ans += i;
    }
    return ans;
}
