#![deny(warnings)]

use std::ops::Range;

fn main() {
    // cheat-sheet https://danielkeep.github.io/itercheat_baked.html

    let nums0to10 = create_nums(0..10);
    let nums100to110 = create_nums(100..110);

    let even_numbers: Vec<String> = // get even numbers in both ranges

    assert_eq!(even_numbers, vec!["2", "4", "6", "8", "100", "102", "104", "106", "108"]);

    let chars_not_0: String = even_numbers. // get chars of even_numbers without 0

    assert_eq!(chars_not_0, String::from("2468112141618"));

    let nums0to10 = create_nums(0..10);
    let nums100to110 = create_nums(100..110);

    let summ: i32 = // calculate the following expression using nums0to10 and nums100to110 iterators
    assert_eq!(summ, (3 * 103) + (4 * 104))
}

fn create_nums(range: Range<i32>) -> impl Iterator<Item=i32> {
    // you should implement generator of iterator from range
}

