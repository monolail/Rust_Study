fn main() {
    let n = 11;
    
    if n > 5 {
        println!("{}은 5보다크다.",n);
    }

    else {
        println!("{}은 5보다 작다.",n);
    }

    // if 응용
    let c = if n > 5 { n+1 } else { n-1};
    println!("c={}",c);

    if n % 4 == 0 {
        println!("{}은 4의 약수이다.",n);
    }

    else if n % 3 == 0 {
        println!("{}은 3의 약수이다.",n);
    }
    
    else if n % 2 == 0 {
        println!("{}은 2의 약수이다.",n);
    }

    else {
        println!("{}은 1의 약수이다.",n);
    }

    let x = 5;

    match x {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        _ => println!("anything")
    }

    let age:u32 = 24;

    let gap = match age {
        0..=10 => "baby",
        11..=19 => "teen",
        20..=30 => "university",
        _ => "adult"
    };

    println!("{}", gap);

    // 튜플
    let n = 33;
    match (n%3, n%5) {
        (0, 0) => println!("3과 5의 배수"),
        (0, _) => println!("3의 배수"),
        (_, 0) => println!("5의 배수"),
        (_, _) => println!("3의 배수도 5의 배수도 아님"),
    }
    let c = 'a';
    if let Some(num) = c.to_digit(10) {
        println!("num={}",num); 
    } else {
        println!("error");   //숫자가 아니므로 이게 출력된다
    }
    
    println!("{}",parse_num('6'));
    println!("{}",parse_num('a'));
}

fn parse_num(c: char) -> u32 {
    let Some(num) = c.to_digit(10) else {
        println!("숫자가 아니다");
        return 0;      //else 블록은 반드시 여기서 끝나야 한다
    };

    //여기서부터 num을 그냥 쓸 수 있다
    num * 2
}
