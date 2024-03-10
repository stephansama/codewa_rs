use std::collections::HashMap;

/// You are given an odd-length array of integers, in which all of them are the same, except for one single number.
///
/// Complete the method which accepts such an array, and returns that single different number.
///
/// The input array will always be valid! (odd-length >= 3)
///
/// Examples
///
/// `[1, 1, 2] ==> 2`
///
/// `[17, 17, 3, 17, 17, 17, 17] ==> 3`
fn stray(arr: &[u32]) -> u32 {
    arr.iter()
        .fold(HashMap::<u32, u8>::new(), |mut acc, c| {
            acc.entry(*c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        })
        .into_iter()
        .find(|i| i.1 == 1)
        .unwrap()
        .0
}

/// smart implementation
///
/// https://www.codewars.com/kata/57f609022f4d534f05000024/solutions/rust
fn stray_2(arr: &[u32]) -> u32 {
    arr.iter().fold(0, |x, i| x ^ i)
}

fn main() {
    stray(&[1, 1, 2]);
    // stray(&[17, 17, 3, 17, 17, 17, 17]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stray() {
        let arr = &[1, 1, 2];
        let expected = 2;
        let actual = stray(arr);
        assert_eq!(expected, actual);
    }
}
