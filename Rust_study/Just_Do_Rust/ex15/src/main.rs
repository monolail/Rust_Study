// ownership

fn main() {
    let a = vec![1,2,3,4];
    println!("a={:?}",a); //a=[1, 2, 3, 4]

    let b = a; // 소유권 전달
    println!("b={:?}",b); //b=[1, 2, 3, 4]

    // let s = String::from("Hello Jeff");
    // let point = cal_point(s);  //소유권 이동 발생

    // //println!("s = {:?}", s);  //에러 발생
    // println!("point = {}", point);  // point = 70

    let mut c = vec![1,2,3];
    println!("c={:?}",c); //c=[1, 2, 3]

    let cnt = get_even_cnt(&c); // 그냥 c이면 에러 발생 -> &을 사용하여 대여만 진행
    println!("even count = {}", cnt);  //even count = 1
    println!("c={:?}",c);  //a=[1, 2, 3]

    add_one(&mut c);  // 쓰기 가능한 대여권 부여
    println!("c={:?}",c);  //[2, 3, 4]

    let s = String::from("hi");

    let r1 = &s;
    let r2 = r1;     //copy. &String은 Copy다
    println!("{} {}", r1, r2);   //둘 다 살아 있다

    let mut v = vec![1,2,3];
    let m1 = &mut v;
    let m2 = m1;     //move. &mut Vec는 Copy가 아니다
    //println!("{:?}", m1);   //error
    println!("{:?}", m2);
}

fn cal_point(w:String) -> i32 {
    let point = w.to_uppercase().bytes()
        .filter(|c| *c >= b'A' && *c <= b'Z')
        .map(|c| (c-b'A') as i32)
        .sum();
    return point;
}

fn get_even_cnt(x:&Vec<i32>) -> i32{
    let mut cnt = 0;
    for i in x {
        if *i % 2 == 0 {cnt += 1;}
    }
    return cnt;
}

fn add_one(x:&mut Vec<i32>) {
    x.iter_mut().for_each(|i| *i += 1);
}