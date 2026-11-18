// 반복 예제[
#[test]
fn simple1(){
    // 1~100까지 합
    let sum:usize = (1..=100).sum();
    println!("sum={}",sum);  //5050

    //1에서 10까지의 곱
    let p:usize = (1..=10).product::<usize>();
    println!("product={}",p);  //3628800

    // 1~100에서 짝수의 합
    let sum:usize = (1..=100).filter(|x| *x % 2 == 0).sum();
    println!("sum={}",sum); //2550

    // 1~100에서 3 혹은 5의 배수의 합
    let sum:usize = (1..=100).filter(|x| *x % 3 == 0 || *x % 5 == 0).sum();
    println!("sum={}",sum);  //2418

    // 1~100에서 3 혹은 5의 배수의 개수
    let cnt:usize = (1..=100).filter(|x| *x % 3 == 0 || *x % 5 == 0).count();
    println!("cnt={}",cnt);  //47
}

#[test]
fn simple2(){
    let v:Vec<usize> = vec![1,2,3,4,5,6,7,8,9,10];

    //count(): 짝수 개수
    let cnt = v.iter().filter(|x| *x %2 == 0).count();
    println!("count={}",cnt); //count=5

    //sum(): 벡터의 모든 원소 합
    let sum:usize = v.iter().sum();
    println!("sum={}",sum); //sum=55

    //product(): 벡터의 모든 원소 곱
    let product1:usize = v.iter().product();
    println!("product={}", product1); //product=3628800

    //filter().product: 원소 중 4보다 작은 값에 대한 곱 : |&x|
    let product:usize= v.iter().filter(|&x| x < &4).product();
    println!("product={}",product); //product=6

    //filter().product(): 원소 중 4보다 작은 값에 대한 곱 : |x|
    let product:usize= v.iter().filter(|x| *x <&4).product();
    println!("product={}",product); //product=6

    //take(n).product(): 원소 중 앞으로부터 3개를 취해서 곱: take(n)   
    let product:usize = v.iter().take(3).product();
    println!("product={}", product); //product=6

    //take_while: filter와 달리, 조건식이 false이면 실행 중단    
    let sum:usize = v.iter().take_while(|x| *x < &5).sum();
    println!("sum={}", sum); //10

    //step_by(n).sum(): 4씩 건너뛰면서 읽어서 합    
    let sum:usize = v.iter().step_by(4).sum();
    println!("sum={}",sum); //sum=15

    //filter().max(): 3의 배수 중 최댓값    
    let max = v.iter().filter(|x| *x %3 ==0).max();
    println!("max={:?}",max); //max=9
}

#[test]
fn middle() {
    // [1..100]에서 10의 배수값들에 대해서 10으로 나눈 후 합
    let sum: u32 = (1..=100)
        // 여기에 코드를 작성하세요
        .filter(|x| x%10 == 0).map(|x| x/ 10)
        .sum();
    println!("sum={}", sum); // 예상 출력: 55

    // [1..100]에서, 10보다 작은 수에서, 3의 배수 합
    let sum: u32 = (1..=100)
        // 여기에 코드를 작성하세요
        .filter(|x| *x < 10 && *x % 3 == 0)
        .sum();
    println!("sum={}", sum); // 예상 출력: 18
}

#[test]
fn string_test1() {
    // map().filter().map(): 문자열 중 숫자로 된 문자열만 골라내서, 해당 문자열을 숫자로 변환해서 벡터로 만듦
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a.iter()
        // 여기에 코드를 작성하세요
        .map(|s| s.parse::<i32>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();
    println!("v={:?}", v); // 예상 출력: v=[1, 5]

    // filter_map(): 위 예제와 동일
    let a = ["1", "two", "NaN", "four", "5"];
    let v: Vec<_> = a.iter()
        // 여기에 코드를 작성하세요
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("v={:?}", v); // 예상 출력: v=[1, 5]

    // 문자 배열을 십진수 숫자 배열로 변환
    let a = ['1', '2', '3', 'f', '5'];
    let v: Vec<_> = a.iter()
        // 여기에 코드를 작성하세요
        .filter_map(|c| c.to_digit(10))
        .collect();
    println!("v={:?}", v); // 예상 출력: v=[1, 2, 3, 5]
}
