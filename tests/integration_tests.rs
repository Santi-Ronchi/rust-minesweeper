use guessing_game::config::run;


#[test]
fn incorrect_field() {
    let check_var = run("minefield_nonexistant.txt".to_string());
    assert!(check_var.is_err());
}

#[test]
fn noformat_field() {
    let check_var = run("minefield_noformat.txt".to_string());
    assert!(check_var.is_err());
}

#[test]
fn correct_field() {
    let check_var = run("minefield.txt".to_string());
    assert!(check_var.is_ok());
}
