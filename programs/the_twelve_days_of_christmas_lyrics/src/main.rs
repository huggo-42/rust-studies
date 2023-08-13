const VERSE_1: &str = "On the first day of Christmas, my true love sent to me";
const VERSE_1_ADD: &str = "A partridge in a pear tree";

const VERSE_2: &str = "On the second day of Christmas, my true love sent to me";
const VERSE_2_ADD: &str = "Two turtle doves, and";

const VERSE_3: &str = "On the third day of Christmas, my true love sent to me";
const VERSE_3_ADD: &str = "Three french hens";

const VERSE_4: &str = "On the fourth day of Christmas, my true love sent to me";
const VERSE_4_ADD: &str = "Four calling birds";

const VERSE_5: &str = "On the fifth day of Christmas, my true love sent to me";
const VERSE_5_ADD: &str = "Five golden rings";

const VERSE_6: &str = "On the sixth day of Christmas, my true love sent to me";
const VERSE_6_ADD: &str = "Six geese a-laying";

const VERSE_7: &str = "On the seventh day of Christmas, my true love sent to me";
const VERSE_7_ADD: &str = "Seven swans a-swimming";

const VERSE_8: &str = "On the eighth day of Christmas, my true love sent to me";
const VERSE_8_ADD: &str = "Eight maids a-milking";

const VERSE_9: &str = "On the ninth day of Christmas, my true love sent to me";
const VERSE_9_ADD: &str = "Nine ladies dancing";

const VERSE_10: &str = "On the tenth day of Christmas, my true love sent to me";
const VERSE_10_ADD: &str = "Ten lords a-leaping";

const VERSE_11: &str = "On the eleventh day of Christmas, my true love sent to me";
const VERSE_11_ADD: &str = "Eleven pipers piping";

const VERSE_12: &str = "On the twelfth day of Christmas, my true love sent to me";
const VERSE_12_ADD: &str = "Twelve drummers drumming";

const ARRAY_VERSES: [&str; 12] = [
    VERSE_1, VERSE_2, VERSE_3, VERSE_4, VERSE_5, VERSE_6, VERSE_7, VERSE_8, VERSE_9, VERSE_10,
    VERSE_11, VERSE_12,
];
const ARRAY_ADD: [&str; 12] = [
    VERSE_1_ADD,
    VERSE_2_ADD,
    VERSE_3_ADD,
    VERSE_4_ADD,
    VERSE_5_ADD,
    VERSE_6_ADD,
    VERSE_7_ADD,
    VERSE_8_ADD,
    VERSE_9_ADD,
    VERSE_10_ADD,
    VERSE_11_ADD,
    VERSE_12_ADD,
];

fn main() {
    let mut i = 0;
    for verse in ARRAY_VERSES.iter() {
        let mut j = i;

        println!("\n[VERSE {}]", i + 1);
        println!("{}", verse);

        for _ in 0..i {
            println!("{}", ARRAY_ADD[j]);
            j -= 1;
        }

        println!("{}", ARRAY_ADD[0]);

        i += 1;
    }
}
