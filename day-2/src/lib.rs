pub mod day2 {
    use std::fs;

    pub struct Contents {
        pub path: String,
        pub contents: String,
    }

    impl Contents {
        pub fn new(path: String) -> Contents {
            let contents: String =
                fs::read_to_string(&path).expect("Something went wrong reading the file");
            Contents { path, contents }
        }
    }

    pub fn parse_contents(contents: String) -> Result<Vec<Vec<i32>>, String> {
        let mut list: Vec<Vec<i32>> = vec![vec![]; contents.lines().count()];
        for (iter, line) in contents.lines().enumerate() {
            for number in line.split_whitespace() {
                match number.parse::<i32>() {
                    Ok(num) => list[iter].push(num),
                    Err(_) => {
                        return Err(format!("Failed to parse number on line {}", iter + 1));
                    }
                }
            }
        }

        Ok(list)
    }

    pub fn count_safe(content: &Vec<Vec<i32>>) -> Result<i32, String> {
        let mut safe_count = 0;

        for line in content.iter() {
            let is_decreasing = line.windows(2).any(|w| w[1] < w[0]);
            let is_increasing = line.windows(2).any(|w| w[1] > w[0]);
            let no_dupe = line.windows(2).any(|w| w[1] == w[0]);
            let within_limits = line
                .windows(2)
                .all(|w| ((w[1] - w[0]).abs() >= 1) && ((w[1] - w[0]).abs() <= 3)); // Changed to `all`

            if is_decreasing == is_increasing {
                continue;
            }

            if no_dupe {
                continue;
            }

            if !within_limits {
                continue;
            }

            safe_count += 1;
        }

        Ok(safe_count)
    }
}
