fn main() {
    for number in 1..=12 {
        println!("{}", verse(number));
    }
}

const ORDINALS: &[&str] = &[
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: &[&str] = &[
    "a Partridge in a Pear Tree",
    "two Turtle Doves",
    "three French Hens",
    "four Calling Birds",
    "five Gold Rings",
    "six Geese-a-Laying",
    "seven Swans-a-Swimming",
    "eight Maids-a-Milking",
    "nine Ladies Dancing",
    "ten Lords-a-Leaping",
    "eleven Pipers Piping",
    "twelve Drummers Drumming",
];

fn verse(number: usize) -> String {
    let mut verse = format!(
        "On the {} day of Christmas my true love gave to me ",
        ORDINALS[number - 1]
    );

    for i in (1..=number).rev() {
        let gift = if i == 1 {
            format!("and {}.", GIFTS[i - 1])
        } else {
            format!("{}, ", GIFTS[i - 1])
        };
        verse += &gift;
    }

    verse
}
