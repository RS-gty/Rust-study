use std::fmt;
use std::fmt::Formatter;

enum ScoreError {
    Empty,
    InvalidFormat(String),
    OutOfRange(i32),
}

impl fmt::Display for ScoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScoreError::Empty => {
                write!(f, "没有成绩")
            }

            ScoreError::InvalidFormat(s) => {
                write!(f, "成绩格式错误: {}", s)
            }

            ScoreError::OutOfRange(n) => {
                write!(f, "成绩超出范围: {}", n)
            }
        }
    }
}


fn main() {
    let tests = [
        "80,90,75,100,85",
        "80,120,90",
        "80,abc,90",
        "80,,90",
        "",
        "100,0",
    ];

    let a = ScoreError::Empty;
    let b = ScoreError::InvalidFormat(String::from("abc"));
    let c = ScoreError::OutOfRange(120);

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn process_scores(input: &str) -> Result<f64, ScoreError> {
    if input.is_empty() {
        return Err(ScoreError::Empty);
    }

    let mut sum = 0;
    let mut size = 0;

    for x in input.split(',') {
        if x.is_empty() {
            return Err(ScoreError::InvalidFormat(String::from("存在空成绩")));
        }

        let n = x.parse::<i32>()
            .map_err(|_| ScoreError::InvalidFormat(String::from(x)))?;

        if n < 0 || n > 100 {
            return Err(ScoreError::OutOfRange(n));
        }

        sum += n;
        size += 1;
    }

    Ok(sum as f64 / size as f64)
}