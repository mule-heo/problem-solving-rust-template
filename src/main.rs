use std::fmt::Write;
use std::io::stdin;

fn main() {
    let mut result = String::new();
    let input = read_line().trim().to_string();

    writeln!(result, "{}", input).unwrap();
    println!("{}", result);
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_result() {
        let result_file_path = "output.txt";
        let result = fs::read_to_string(result_file_path).expect("No such file or directory.");

        let answer_file_path = "expected-output.txt";
        let answer = fs::read_to_string(answer_file_path).expect("No such file or directory.");

        assert_eq!(result.trim(), answer.trim());
    }
}
