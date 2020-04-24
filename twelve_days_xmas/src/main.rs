fn main() {
    #[rustfmt::skip]
    let days = vec![
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    // This vec could be constructed in reverse order to avoid the call to .rev()
    let gifts = vec![
        "partridge in a pear tree",
        "Turtle doves",
        "French hens",
        "Calling birds",
        "Gold rings",
        "Geese a-laying",
        "Swans a-swimming",
        "Maids a-milking",
        "Ladies dancing",
        "Lords a-leaping",
        "Pipers piping",
        "Drummers drumming",
    ];

    for (i, day) in (1..).zip(days.iter()) {
        println!("On the {} day of Christmas my true love sent to me", day);

        let first_gift_start = if i == 1 { "A" } else { "And a" };

        // This loop is needlessly complex because the gifts text
        // doesn't include the count e.g. "2 French hens"
        // if it did then the .zip() and index calculation wouldn't be required
        for (j, gift) in (1..).zip(gifts.iter().take(i).rev()) {
            println!(
                "{} {}",
                if i == j {
                    first_gift_start.to_string()
                } else {
                    (i - j + 1).to_string()
                },
                gift
            );
        }
        println!();
    }
}
