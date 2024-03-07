use rand::random;

fn main() {
    let stdin = std::io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let output: &str = [
            r#"{"gesture": "Rock"}"#,
            r#"{"gesture": "Scissors"}"#,
            r#"{"gesture": "Paper"}"#,
        ][random::<usize>() % 3];
        println!("{}", output);
    }
}
