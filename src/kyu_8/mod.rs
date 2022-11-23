fn rock_paper_scissors(p1: &str, p2: &str) -> &'static str {
    if p1 == p2 {
        return "Draw!";
    }
    match (p1, p2) {
        ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
        _ => "Player 2 won!",
    }
}