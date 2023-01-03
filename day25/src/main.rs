fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate().fold(0, |acc, (idx, c)| {
        let m: i64 = match c {
            '=' => -2,
            '-' => -1,
            '0' | '1' | '2' => c.to_digit(10).unwrap() as i64,
            _ => panic!(),
        };
        acc + (m * 5_i64.pow(idx as u32))
    })
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut sum = 0;
    let mut power = 0;
    while sum < decimal {
        sum += 2 * 5_i64.pow(power);
        power += 1;
    }

    let mut elements = vec![2; power as usize];
    for (idx, element) in elements.iter_mut().enumerate() {
        loop {
            sum -= 5_i64.pow(power - (idx + 1) as u32);
            if sum < decimal {
                sum += 5_i64.pow(power - (idx + 1) as u32);
                break;
            } else if sum == decimal {
                *element -= 1;
                break;
            } else {
                *element -= 1;
            }
        }
    }

    elements
        .iter()
        .map(|e| match e {
            2 | 1 | 0 => char::from_digit(*e as u32, 10).unwrap(),
            -1 => '-',
            -2 => '=',
            _ => panic!("error"),
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let sum = input.lines().map(|snafu| snafu_to_decimal(snafu)).sum();
    let snafu = decimal_to_snafu(sum);
    assert_eq!(snafu, "2-02===-21---2002==0");
}
