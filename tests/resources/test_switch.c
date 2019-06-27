int main() {
    int a = 5;
    int ans = 0;
    switch(a) {
        case 1:
            ans += 1;
        case 5:
            ans += 5;
        case 10:
            ans += 10;
        default:
            ans += 100;
    }
    return ans;
}
