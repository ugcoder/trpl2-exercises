use unicode_segmentation::UnicodeSegmentation;

fn pig_latin(word: &str) -> String {
    let first_str = word.graphemes(true).next().unwrap();
    let is_vowel = VOWEL.contains(&first_str);
    if is_vowel {
        // println!("\nIt's VOWEL");
        format!("{}-{}", word, "hay")
    } else {
        // println!("\nNot VOWEL");
        let first_index = word
            .grapheme_indices(true)
            .find(|(_, c)| VOWEL.contains(c))
            .map(|(i, _)| i)
            .unwrap_or(word.len());

        let prefix = &word[..first_index];
        let suffix = &word[first_index..];
        format!("{}-{}{}", suffix, prefix, "ay")
    }
}

const VOWEL: &'static str = "aeiouAEIOU";

fn main() {
    let sentence = "hello apple first string café igloo";

    let words = sentence.split_whitespace().collect::<Vec<&str>>();

    for word in words {
        println!(
            "The pig Latin translation of {} is: {}",
            word,
            pig_latin(word)
        )
    }
}
