const VOWELS: &'static [&'static str] = &["a", "e", "i", "o", "u"];

// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn to_pig_latin(word: &str) -> String {
    let first_letter = word.chars().next().unwrap().to_string();
    if !VOWELS.contains(&&first_letter[..]) {
        let word_end = String::from(word[1..].to_string());
        return word_end + "-" + &first_letter[..] + "ay";
    } else {
        return word.to_string() + "-hay";
    }
}

fn main() {
    let words = vec!["first", "apple"];

    for word in words {
        println!("{}", to_pig_latin(word));
    }
}
