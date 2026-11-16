fn main() {
    let mut v:Vec<i32> = Vec::new();
    v.push(1); v.push(2);
    println!("{:?}",v);  //[1, 2]

    let mut v2 = vec![];
    v2.push(1); v2.push(2);
    println!("{:?}",v2);  //[1, 2]
    
    let arr = [3,4];
    let v3 = arr.to_vec();
    println!("{:?}",v3);

    // 벡터 인덱스
    let v = vec![5,6,7];
    println!("v[0]={}", v[0]);
    println!("last element={}", v[v.len()-1]);

    //get 이용한 접근
    let v = vec![1,2,3];
    // println!("{}",v[3]);  //panic 발생-> 인덱스 넘어섬
    println!("{:?}", v.get(3));  //None 출력 -> 인덱스 넘어섬ㄴ

    if let Some(n) = v.get(1) {
        println!("v.get(1)={}",n);
    }

    let v = vec![1,2,3];
    //레퍼런스로 접근하기
    for a in &v{ print!("{} ",*a); }  //5 6 7
    println!("");

    for a in &v{ print!("{} ",a); }  //5 6 7  print! 매크로는 레퍼런스 변수에 대해서도 안의 내용을 출력함
    println!("");

    //소유권 이동
    for a in v{ print!("{} ",a); }  //5 6 7
    println!("");

    //println!("{}",v[0]); //error

    let mut v = vec![1,2,3];
    println!("{:?}",v);
    for i in &mut v {
        *i *= 2;
    }

    println!("{:?}",v);

    println!("{}", v[0]);
    println!("{}, {}", v.len(), v[v.len()-1]);

    // println!("{}",v[3]);  //panic 발생
    println!("{:?}", v.get(3));  //None 출력

    if let Some(n) = v.get(1) {
        println!("v.get(1)={}",n);
    }

    for a in &v{print!("{} ",*a);}

    for a in &v{ print!("{} ",a); }  //5 6 7  print! 매크로는 레퍼런스 변수에 대해서도 안의 내용을 출력함

    for a in &mut v { *a *= 2; }
    println!("{:?}",v);

    println!("");

    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];

    vec.append(&mut vec2);
    println!("{:?}",vec);
    // vec.clear();

    println!("{:?}",vec);
    

    vec.resize(2,0); // [1,2]
    println!("{:?}",vec);
    vec.resize(4, 4); // [0,0,4,4]
    println!("{:?}",vec);

    vec.fill(1);
    println!("{:?}",vec);
    vec.truncate(3);
    println!("{:?}",vec);

}
