use number_to_words::number_to_words;

fn main() {
    const COUNTING: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const LYRICS: [&str; 12] = [
        "a Partridge in a pear tree.",
        "turtle doves,",
        "French Hens,",
        "calling birds,",
        "golden rings,",
        "geese a laying,",
        "swans a swimming,",
        "maids a milking,",
        "ladies dancing,",
        "lords a leaping,",
        "pipers piping,",
        "drummers drumming,",
    ];

    println!("The Twelve Days of Christmas!\n");

    for i in 0..COUNTING.len() {
        println!("On the {} day of Christmas", COUNTING[i]);
        println!("My true love sent to me:");

        if i == 0 {
            println!("A partridge in a pear tree.");
        }

        for j in (1..i + 1).rev().step_by(1) {
            println!("{} {}", number_to_words(j as f64 + 1.0, true), LYRICS[j]);
        }

        if i >= 1 {
            println!("And {}", LYRICS[0]);
        }
        println!();
    }
}
