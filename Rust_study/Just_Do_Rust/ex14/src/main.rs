
#[test]
fn string_test2() {
    // 앞 5개 문자만 잘라내서, 숫자로 변환
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let v: Vec<u32> = input.chars()
        // 여기에 코드를 작성하세요
        .take(5)
        .filter_map(|c| c.to_digit(10))
        .collect();
    println!("{:?}", v); // 예상 출력: [7, 3, 1, 6, 7]

    // 5번째 숫자부터 5개 문자를 잘라내서, 숫자로 변환
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let v: Vec<u32> = input[4..9].chars()
        // 여기에 코드를 작성하세요
        .filter_map(|c| c.to_digit(10))
        .collect();
    println!("{:?}", v); // 예상 출력: [7, 1, 7, 6, 5]

    // 숫자로 모두 변환 후 짝수만 더하기
    let input: &str = "73167176531330624919225119674426574742355349194934";
    let sum: u32 = input.chars()
        // 여기에 코드를 작성하세요
        .filter_map(|c| c.to_digit(10))
        .filter(|c| *c % 2 == 0)
        .sum();
    println!("sum={}", sum); // 예상 출력: 72
}

