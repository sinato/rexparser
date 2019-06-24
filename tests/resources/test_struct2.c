int main() {
    struct s {
        int a1;
        int a2;
    };
    struct s st;
    st.a1 = 33;
    st.a2 = 22;
    return st.a1 + st.a2;
}
