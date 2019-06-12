int main() {
    int a = 0;
    int ans = 0;
    while(a < 10) {
        a++;
        ans++;
        if (a == 5) {
            ans = 100;
            break;
        }
    }
    return ans;
}
