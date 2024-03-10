fn reverse_words(str: &str) -> String {
    str.to_string()
        .split(' ')
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let str = "This is an example!";
    println!("{}", reverse_words(str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let mut str = "This is an example!";
        assert_eq!("sihT si na !elpmaxe", reverse_words(str));
        str = "double  spaces";
        assert_eq!("elbuod  secaps", reverse_words(str));
    }
}
