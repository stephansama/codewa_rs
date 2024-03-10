fn high_and_low(numbers: &str) -> String {
    let mut split_numbers = numbers
        .split(' ')
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    split_numbers.sort();

    let lowest = split_numbers.first().unwrap();
    let highest = split_numbers.last().unwrap();

    format!("{highest} {lowest}")
}

fn main() {
    let ans = high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4");
    println!("{ans:?}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_test_1() {
        println!("Halo!");
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }

    #[test]
    fn example_test_2() {
        assert_eq!("3 1", high_and_low("1 2 3"));
    }
}
