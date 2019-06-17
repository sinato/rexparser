int print_board(int board[8][8]) {
    int ans = 0;
    for (int i = 0; i < 8; i++) {
        for(int j = 0; j < 8; j++)
            if (board[i][j])
                putchar('Q');
            else
                putchar('.');
        putchar(10);
    }
    putchar(10);
    return 0;
}

int conflict(int board[8][8], int row, int col) {
    for (int i = 0; i < row; i++) {
        if (board[i][col])
            return 1;
        int j = row - i;
        if (0 < col - j + 1 && board[i][col - j])
            return 1;
        if (col + j < 8 && board[i][col + j])
            return 1;
    }
    return 0;
}

int solve(int board[8][8], int row) {
    if (row > 7) {
        print_board(board);
        return 0;
    }
    for (int i = 0; i < 8; i++) {
        if (conflict(board, row, i)) {
        } else {
            board[row][i] = 1;
            solve(board, row + 1);
            board[row][i] = 0;
        }
    }
    return 1;
}

int main() {
    int board[8][8];
    for (int i = 0; i < 8; i++)
        for (int j = 0; j < 8; j++)
            board[i][j] = 0;
    solve(board, 0);
    return 3;
}

