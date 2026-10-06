fn main(){
    // 출력의 종류
    print!("Hello, "); // print!()는 줄바꿈을 하지 않는다.
    println!("world!"); // println!()는 줄바꿈을 한다.

    //r# : raw string literal : 있는 그대로 출력
    // 용도:
    let my_string = r#"Hello, "world"!"#;
    println!("{}", my_string);
    let my_string2 = "Hello, \"world\"!";
    println!("{}", my_string2); // my_string과 my_string2는 같은 문자열을 출력한다. -> r#은 가독성을 높인다.

    let r#let = "Hello, world!";
    println!("{}", r#let); // r#을 붙이면 예약어를 변수명으로 사용할 수 있다.

    let my_function = r#return();
    println!("Function returned: {}", my_function);

    // {}의 응용
    println!("{} + {} = {}", 1, 2, 1 + 2); // {}는 순서대로 매개변수를 출력한다.
    // "2 + 1 = 3"을 출력됨
    println!("{1} + {0} = {2}", 1, 2, 1 + 2); // {}안에 숫자를 넣으면 매개변수의 순서를 바꿀 수 있다. -> 인덱스 기반 포맷팅
    println!("{b} + {a} = {c}", a=1, b=2, c=1+2); // {}안에 이름을 넣으면 매개변수의 순서를 바꿀 수 있다. -> 변수 이름 기반 포맷팅

    // 문자열
    // &str : 간단한 문자열 -> 속도가 빠르다
    let name = "Alice";
    // String : 복잡한 문자열 -> 속도가 느리지만 다양한 기능 제공.
    let other_name = String::from("Alice");

    // const와 static 차이
    const MAX_POINTS: u32 = 100;
    println!("MAX_POINTS: {}", MAX_POINTS);
    static MAX_POINTS2: u32 = 100;
    println!("MAX_POINTS2: {}", MAX_POINTS2);
    // const는 컴파일 타임에 값이 결정되지만, static은 런타임에 값이 결정된다. -> const는 상수, static은 전역 변수(메모리 위치 고정)

    // 참조
    let country = String::from("Korea");
    let country_ref = &country; // 참조를 통해 country의 값을 가져온다
    println!("Country: {}", country_ref);

    // 변경 가능한 참조 -> mut를 사용
    let mut country2 = String::from("Korea");
    let country_ref2 = &mut country2; // 변경 가능한 참조를 통해 country2의 값을 가져온다
    println!("Country2: {}", country_ref2);

    // 섀도잉 -> 값을 소멸하는 것이 아니라, 같은 이름의 변수를 새로 선언하여 기존 변수를 가리는 것
    let x = 5;
    let tep = &x;
    let x = 6; // x를 새로 선언하여 기존 x를 가린다.
    let tep2 = &x;
    println!("tep: {}, tep2: {}", tep, tep2); // tep는 5, tep2는 6을 출력한다. -> 섀도잉은 참조를 통해 값을 가져올 때, 기존 변수를 가리기 때문에 기존 변수의 값이 변경되지 않는다.

    // 함수에 대한 참조 제공
    let country3 = String::from("Korea");
    print_country(country3); // country3의 소유권이 print_country 함수로 이동
    // print_country(country3); // country3의 소유권이 print_country 함수로 이동했기 때문에, country3은 더 이상 사용할 수 없다. -> 소유권 이동
    let country4 = String::from("Korea");
    print_country(country4); // country4의 소유권을 이동하지 않고, 참조를 통해 값을 가져온다. -> 소유권 이동이 일어나지 않음
    print_country(country4); // country4는 여전히 사용할 수 있다. -> 소유권 이동이 일어나지 않았기 때문에, country4는 여전히 유효하다.
}

// 함수에도 적용가능한 r#
fn r#return() -> u8{
    println!("Here is my number");
    1
}

fn print_country(country: &str){
    println!("Country: {}", country);
}