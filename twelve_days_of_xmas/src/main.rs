const GIFTS: &'static [&'static str] = &[
    "partridge in a pear tree",
    "turtle doves",
    "French hens",
    "calling birds",
    "gold rings",
    "geese a laying",
    "swans a swimming",
    "maids a milking",
    "ladies dancing",
    "lords a leaping",
    "pipers piping",
    "drummers drumming",
];

const EN_CARDINAL_NUMBERS: &'static [&'static str] = &[
    "A", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve",
];

const EN_ORDINAL_NUMBERS: &'static [&'static str] = &[
    "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eightth", "Ninth", "Tenth",
    "Eleventh", "Twelfth",
];

fn verse(n: usize, lyrics: &mut String) {
    lyrics.push_str("On the ");
    lyrics.push_str(EN_ORDINAL_NUMBERS[n]);
    lyrics.push_str(" day of Christmas, my true love sent to me\n");
    for i in (0..n + 1).rev() {
        lyrics.push_str(EN_CARDINAL_NUMBERS[i]);
        lyrics.push_str(" ");
        lyrics.push_str(GIFTS[i]);
        if i >= 1 {
            lyrics.push_str(", ");
        }
        if i == 1 {
            lyrics.push_str("and ");
        }
        lyrics.push_str("\n");
    }
    lyrics.push_str("\n\n");
}

fn main() {
    let mut chorus = String::new();
    for i in 0..12 {
        verse(i, &mut chorus);
    }
    println!("{}", chorus);
}
