pub fn scrabble_points(word: String) -> u8 {
    let mut points: u8 = 0;

    word.to_lowercase()
        .chars()
        .into_iter()
        .map(|c| points += letter_to_points(c))
        .count();

    points
}

fn letter_to_points(c: char) -> u8 {
    match c {
        'a' | 'e' | 'i' | 'l' | 'n' | 'o' | 'r' | 's' | 't' | 'u' => return 1,
        'd' | 'g' => return 2,
        'b' | 'c' | 'm' | 'p' => return 3,
        'f' | 'h' | 'v' | 'w' | 'y' => return 4,
        'k' => return 5,
        'j' | 'x' => return 8,
        'q' | 'z' => return 10,
        _ => return 0,
    }
}
