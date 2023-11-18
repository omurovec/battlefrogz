struct Frog {
    jmp: u8,
    spd: u8,
    int: u8,
    bty: u8,
}

enum Result {
    ClientWin,
    ClientLoss,
    Tie,
}

// SCORING FUNCTIONS

pub fn sum_score(frog: Frog) -> u8 {
    frog.jmp + frog.spd + frog.int + frog.bty
}

pub fn dominance(int_a: u8, spd_b: u8) -> u8 {
    if int_a > spd_b {
        1u8 // Intellect of frog A negates some speed of frog B
    } else {
        0u8
    }
}

pub fn jump_threshold(jmp: u8) -> u8 {
    if (jmp > 15u8) {
        2u8 // Bonus for high jump
    } else {
        0u8
    }
}

pub fn get_score(frog: Frog, spd_other: u8) -> u8 {
    sum_score(frog) + dominance(frog.int, spd_other) + jump_threshold(frog.jmp)
}

// GAME LOGIC

pub fn main(frog_0: Frog, frog_1: Frog) -> Result {
    let score_0 = get_score(frog_0, frog_1.spd);
    let score_1 = get_score(frog_1, frog_0.spd);
    // Determine the winner
    if score_0 == score_1 {
        // Tie-breaker based on intellect
        if frog_0.int > frog_1.int {
            Result::ClientLoss // Frog 0 wins
        } else if frog_1.int > frog_0.int {
            Result::ClientWin // Frog 1 wins
        } else {
            Result::Tie // Tie
        }
    } else if score_0 > score_1 {
        Result::ClientLoss // Frog 0 wins
    } else {
        Result::ClientWin // Frog 1 wins
    }
}
