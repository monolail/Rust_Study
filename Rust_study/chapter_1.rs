// 책으로 쓰는 타입 - documentaion
struct Book;

fn main(){
    // 이것은 주석이다 : comments
    println!("Hello world");
    let x /* 이 변수는 16비트 정수입니다 */ = 10;

    // Primitive types 원시 타입
    // + Plus sign
    // - minus sign

    // 부호 있다. : i8, i16, i32, i64, i128, and isize
    // 부호 없다. : u8, u16, u32, u64, u128, and usize
    /* i8은 1바이트 i64는 8바이트
        i8는 -128에서 127까지 , u8는 0에서 255까지 */
    // isize와 usize에 대한 정보
    // computer architecture에 따라 달라진다. 
    // 32bit 컴퓨터에서는 isize와 usize는 32bit이고, 64bit 컴퓨터에서는 64bit이다.

    // 정수 타입
    let my_number:u8 = 100; // 255
    let my_other_number:u16 = 200; // 65,535
    let third_number:u32 = my_number as u32 + my_other_number as u32;
    
    // 타입 변환
    // as 키워드를 사용하여 타입을 변환할 수 있습니다.
    println!("{} {}", my_number, my_number as u8 as char);

    // .len() 문자열 길이구하기
    // .len()은 실제 길이가 아닌 바이트 수를 구한다는 점을 명심
    // chars().count() 은 실제 문자열 길이 구하기
    let my_string = "Hello, world!";
    println!("String length: {}", my_string.len());
    println!("String character count: {}", my_string.chars().count());
    let my_korean_string = "안녕하세요!!";
    println!("Korean String length: {}", my_korean_string.len());
    println!("Korean String character count: {}", my_korean_string.chars().count());

    // 타입 추론
    let number = 42; // Rust는 이 변수를 i32로 추론합니다. -> 굳이 타입을 명시하지 않아도 된다.
    let number2 :u32 = 42; // 명시적으로 타입을 지정할 수도 있다.
    let number3 = 42u32; // 숫자 뒤에 u32를 붙여서 타입을 지정할 수도 있다.
    let number4 = 42_u32; // 숫자 뒤에 _를 붙여서 타입을 지정할 수도 있다. -> 밑줄의 갯수는 상관없다.


    // 실수 -> 러스트는 기본적으로 f64를 사용한다.
    let my_float:f32 = 3.14; // 32비트 부동소수점
    let my_other_float:f64 = 3.14; // 64비트 부동소수점

    // 출력문
    // {}는 변수를 출력할 때 사용한다.
    println!("My number is: {}", my_number);
    println!("The sum of 5 and 10 is: {}", add(5, 10));
    
    println!("The my_float is: {}", my_float);
    //{}에 변수명을 직접 넣을 수도 있다. -> {}안에 변수명을 넣으면 해당 변수를 출력한다.
    println!("The my_float is: {my_float}");

    // Rust의 가장 큰수와 작은수
    println!("The largest i32 is: {}", std::i32::MAX);
    println!("The smallest i32 is: {}", std::i32::MIN);

    // 변수 선언의 let
    let my_variable = 10; // 변수 선언
    println!("The value of my_variable is: {}", my_variable);
    // let의 경우 변수의 값을 변경할 수 없다. -> 불변 변수
    // my_variable = 20; // 오류 발생

    // 섀도잉 -> 현재 my_variable을 새로운 값으로 덮어씌우는 것
    // 현 시점 에서 my_variable은 10이지만, 아래 let문으로 인해 my_variable은 20으로 변경된다.
    // local scope에서만 유효하다.
    let my_variable = 20; // 변수 선언
    println!("The value of my_variable is: {}", my_variable);

    

}

// Rust함수의 마지막 부분이 곧 반환값이다. return 키워드를 사용하지 않아도 된다.
 // -> i32는 32비트 정수형을 의미한다.
fn add(x: i32, y: i32) -> i32 {
        x + y
}