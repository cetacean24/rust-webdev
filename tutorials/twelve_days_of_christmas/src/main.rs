use std::io::Write;

const VERSE_1: &str = "A partridge in a pear tree";
const VERSE_2: &str = "Two turtle doves and";
const VERSE_3: &str = "Three french hens";
const VERSE_4: &str = "Four calling birds";
const VERSE_5: &str = "Five golden rings";
const VERSE_6: &str = "Six geese-a-laying";
const VERSE_7: &str = "Seven swans a-swimming";
const VERSE_8: &str = "Eight maids a-milking";
const VERSE_9: &str = "Nine ladies dancing";
const VERSE_10: &str = "Ten lords a-leaping";
const VERSE_11: &str = "Eleven pipers piping";
const VERSE_12: &str = "Twelve drummers drumming";

const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

fn enter_to_continue(){
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn main() {

    // verse goes from 1 to 12
    let mut verse: usize = 0;

    // vector to hold the lines that are to be sung each iteration
    let mut sung_verses: Vec<&str> = Vec::new();

    // iterate through the 12 days; could also use for verse in 1..=12
    while verse < 12 {
        
        // increment verse after the loop begins
        verse += 1;

        // get the current day
        let current_day: &str = DAYS[verse - 1]; // adjust for zero indexing
        
        println!("On the {} day of Christmas, my true love gave to me", current_day);

        // add the current verse to the sung_verses vector
        match verse {
            1 => sung_verses.push(VERSE_1),
            2 => sung_verses.push(VERSE_2),
            3 => sung_verses.push(VERSE_3),
            4 => sung_verses.push(VERSE_4),
            5 => sung_verses.push(VERSE_5),
            6 => sung_verses.push(VERSE_6),
            7 => sung_verses.push(VERSE_7),
            8 => sung_verses.push(VERSE_8),
            9 => sung_verses.push(VERSE_9),
            10 => sung_verses.push(VERSE_10),
            11 => sung_verses.push(VERSE_11),
            12 => sung_verses.push(VERSE_12),
            _ => (),
        }

        // print the verses in reverse order (oldest at the back, newest at the front)
        for line in sung_verses.iter().rev() {
            println!("{}", line);
        }

        // press enter to continue to next day
        enter_to_continue();
    }
}
