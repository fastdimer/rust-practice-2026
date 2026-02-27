#[allow(dead_code)]
pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {

    let apple_count = apples
        .iter()
        .filter(|&&d| a + d >= s && a + d <= t)
        .count() as i32;

    let orange_count = oranges
        .iter()
        .filter(|&&d| b + d >= s && b + d <= t)
        .count() as i32;

    println!("{}", apple_count);
    println!("{}", orange_count);

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];
        let result = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);

        assert_eq!(result, (1, 1));
    }

    #[test]
    fn test_no_fruits_on_house() {
        let apples = vec![-10, -5];
        let oranges = vec![10, 20];
        let result = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);
        assert_eq!(result, (0, 0));
    }
}