use std::fs;

fn main() {
    let path = "src/input.txt";
    let contents = Contents::new(path.to_string());

    /* Part 1 */
    let (list1, list2) = parse_file(contents.contents);

    let total = total_difference(&list1, &list2);
    println!("Total difference: {}", total);

    /* Part 2 */
    let similarity = total_similarity(&list1, &list2);
    println!("Total similarity: {}", similarity);
}

struct Contents {
    path: String,
    contents: String,
}

impl Contents {
    fn new(path: String) -> Contents {
        let contents: String =
            fs::read_to_string(&path).expect("Something went wrong reading the file");
        Contents { path, contents }
    }
}

fn parse_file(contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        list1.push(words[0].parse().unwrap());
        list2.push(words[1].parse().unwrap());
    }

    return (list1, list2);
}

fn total_difference(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut copy1 = list1.clone();
    let mut copy2 = list2.clone();
    copy1.sort();
    copy2.sort();
    let mut total: i32 = 0;

    for num in copy1.iter().zip(copy2.iter()) {
        let (ai, bi) = num;
        if ai > bi {
            total += ai - bi;
        } else {
            total += bi - ai;
        }
    }

    return total;
}

fn total_similarity(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut copy1 = list1.clone();
    let mut copy2 = list2.clone();
    copy1.sort();
    copy2.sort();

    let mut total: i32 = 0;

    for num in copy1.iter() {
        let occurence = copy2.iter().filter(|&n| n == num).count() as i32;
        total += occurence * num;
    }

    return total;
}
