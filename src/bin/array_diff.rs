/*
 * Your goal in this kata is to implement a difference function, which subtracts one list from another and returns the result.
 *
 * It should remove all values from list a, which are present in list b keeping their order.
 *
 * array_diff(vec![1,2], vec![1]) == vec![2]
 *
 * If a value is present in b, all of its occurrences must be removed from the other:
 * array_diff(vec![1,2,2,2,3], vec![2]) == vec![1,3]
 **/

fn array_diff3<T: PartialEq>(start: Vec<T>, subtract: Vec<T>) -> Vec<T> {
    start
        .into_iter()
        .filter(|f| !subtract.contains(f))
        .collect()
}

fn array_diff2<T: PartialEq>(mut start: Vec<T>, subtract: Vec<T>) -> Vec<T> {
    start.retain(|x| !subtract.contains(x));
    start
}

fn array_diff4<T>(start: Vec<T>, subtract: Vec<T>) -> Vec<T>
where
    T: PartialEq + Copy,
{
    let mut ans = vec![];
    start.iter().for_each(|s| {
        if !subtract.contains(s) {
            ans.push(*s)
        }
    });
    ans
}
fn array_diff<T: PartialEq + Copy>(start: Vec<T>, subtract: Vec<T>) -> Vec<T> {
    let mut ans = vec![];

    start.iter().for_each(|s| {
        if !subtract.contains(s) {
            ans.push(*s)
        }
    });

    ans
}

fn main() {
    todo!();
}

// Add your tests here
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
        assert_eq!(array_diff(vec![1, 2, 3], vec![1, 2]), vec![3]);
    }
}
