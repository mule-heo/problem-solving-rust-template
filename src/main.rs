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

fn read_nums_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let nums = read_line()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<_>>();
    nums
}

fn read_nums_multi_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut nums = vec![];
    loop {
        let input = read_line();
        if input.trim().is_empty() {
            break;
        }
        let num = input.trim().parse::<T>().unwrap();
        nums.push(num);
    }
    nums
}

fn read_nums_multi_lines(n: usize) -> Vec<usize> {
    let mut vec = vec![];

    for _ in 0..n {
        let num = read_line().trim().parse::<usize>().unwrap();
        vec.push(num);
    }
    vec
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
