#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(range =>
        (version: "0.0.1")
        (author: "Darrell Hamilton <darrell.noice@gmail.com>")
        (about: "Prints a bunch of numbers")
        (@arg START: +takes_value "Start of range, inclusive. Defaults to 0")
        (@arg END: +takes_value "End of the range, exclusive. Defaults to 10")
        (@arg STEP: +takes_value "Step count between numbers. Defaults to 1")
    ).get_matches();

    let start: i32 = matches.value_of("START").unwrap_or("0").parse().unwrap();
    let end: i32 = matches.value_of("END").unwrap_or("10").parse().unwrap();
    let step: i32 = matches.value_of("STEP").unwrap_or("1").parse().unwrap();

    let mut i = start;

    while i < end {
        println!("{}", i);
        i = i + step;
    }
}
