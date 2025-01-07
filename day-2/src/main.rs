use core::panic;

use lib::day2;

fn main() {
    let path = "src/input.txt";
    let contents = day2::Contents::new(path.to_string());

    let list = match day2::parse_contents(contents.contents) {
        Ok(list) => list,
        Err(e) => panic!("{e}"),
    };

    let safe_count = match day2::count_safe(&list) {
        Ok(num) => num,
        Err(e) => panic!("{e}"),
    };

    println!("Safe: {:?}", safe_count);
}
