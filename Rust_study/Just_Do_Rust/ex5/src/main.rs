struct Point {
    x: i32,    y: i32
}

impl Point {
    fn new(x:i32, y:i32) -> Point{  //1. 연관 함수
        Point {x:x, y:y}
    }

    fn distance(&self, p:&Point) -> f64{  //2. 메서드
        (((p.x-self.x).pow(2) + (p.y-self.y).pow(2)) as f64).sqrt()
    }
}

fn main(){
    let p1 = Point::new(0,0);    let p2 = Point::new(3,4);  //3. 연관 함수의 사용
    assert_eq!(5.0, p1.distance(&p2));  //4. 메서드의 사용

    // closure 클로저
    let add_one = |x| x+1;
    println!("{}",add_one(3));

    let gap = true;

    let k = gap.then(|| 100);
    let k2 = gap.then_some(100);
    let k3 = false.then_some(100);
    println!("{:?},{:?},{:?}",k,k2,k3);


}
