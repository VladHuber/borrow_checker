fn main() {
    assert_eq!(true, solution("abc", "c"));
    assert_eq!(true, solution("abc", "D"));
}

fn solution(word: &str, ending: &str) -> bool {
    // word[0..ending.len()];
    println!("{}", word.ends_with(ending));
    word.ends_with(ending)
}