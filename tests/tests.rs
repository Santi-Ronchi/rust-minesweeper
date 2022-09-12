use guessing_game

mod tests {
    use super::*;

    #[test]
    fn no_mines() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}