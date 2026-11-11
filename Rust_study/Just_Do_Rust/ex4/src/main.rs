fn main() {
    // 튜플
    let p:(&str, u32) = ("hello", 5);
    println!("{:?}", p);

    let info = get_info();
    println!("{},{}",info.0,info.1);

    let a:[i32; 5] = [1,2,3,4,5];

    println!("{:?}",a);
    println!("{}",a[0]);
    println!("{:?}",&a[1..]);
    println!("{:?}",&a[..]);

    let b = [1;5];
    println!("{:?}",b);

    // 벡터 -> 값이 다르면 에러 발생
    let mut v = vec![3,4,5];
    v.insert(3,4);
    println!("{:?}",v);

    println!("{}",add(4,5));

}


// 함수 : fn으로 선언된다.
fn get_info() -> (i32,f64){
    let age = 20;
    let height = 180.4;

    return (age,height);
}

fn add(a:i32,b:i32) -> i32 {
    return a+b;
}