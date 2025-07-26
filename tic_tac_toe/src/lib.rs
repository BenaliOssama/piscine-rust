pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return String::from("player X won");
    }else if diagonals('Y', table) || horizontal('Y', table) || vertical('Y', table) {
        return String::from("player Y won");
    }else{
        return String::from("tie"); 
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    return (((table[0][0] == table[2][2]) && table[0][0] ==table[1][1])
        ||  ((table[2][0] == table[0][2])&& table[2][0] == table[1][1]))
        && table[1][1] ==player 
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3{
        if (table[i][0] == table[i][1] && table[i][1] == table[i][2]) && table[i][2] == player{
            return true ; 
        }
    }
    return false ; 
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3{
        if (table[0][i] == table[1][i] && table[1][i] == table[2][i] && table[2][i] == player){
            return true ; 
        }
    }
    return false ; 
}
