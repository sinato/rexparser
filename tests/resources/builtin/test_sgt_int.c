int main() {
    int ans = 7;
    if(sgt_int(1, 0)) {
        ans = ans + 1;
    }
    if(sgt_int(0, 1)) {
        ans = ans + 10;
    }
    if(sgt_int(0, 0)) {
        ans = ans + 100;
    }
    return ans;
}
