#[allow(dead_code)]
pub fn staircase(n: i32) {
    for i in 1..=n {
        println!("{}", generate_line(n, i));
    }
}

#[allow(dead_code)]
pub fn generate_line(n: i32, i: i32) -> String {
    let spaces = " ".repeat((n - i) as usize);
    let hashes = "#".repeat(i as usize);
    format!("{}{}", spaces, hashes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_line_standard() {
        assert_eq!(generate_line(4, 1), "   #");
        assert_eq!(generate_line(4, 2), "  ##");
        assert_eq!(generate_line(4, 3), " ###");
        assert_eq!(generate_line(4, 4), "####");
    }

    #[test]
    fn test_generate_line_large() {
        assert_eq!(generate_line(6, 6), "######");
        assert_eq!(generate_line(6, 1), "     #");
    }

    #[test]
    fn test_generate_line_single() {
        assert_eq!(generate_line(1, 1), "#");
    }
}