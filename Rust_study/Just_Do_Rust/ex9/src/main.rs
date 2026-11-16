use std::collections::HashMap;

fn main() {
    let mut map:HashMap<&str, i32> = HashMap::new();

    map.insert("kim", 24);
    map.insert("lee", 20);
    println!("{:?}",map);  

    let map2 = HashMap::from([
        ("Jeff", 100), ("Tom", 90),
    ]);
    println!("{:?}",map2);

    let vec = vec!["gang", "Baek"];
    let vec2 = vec![23, 32];
    println!("{:?}",vec);
    println!("{:?}",vec2);

    let map3:HashMap<_,_> = vec.into_iter().zip(vec2.into_iter()).collect();
    println!("{:?}",map3);
    for (k, val) in &map3 {
        println!("{}: {}", k, val);
    }

    let text = "Hello World";
    let mut text_map = HashMap::new();

    for c in text.chars() {
        let cnt = text_map.entry(c).or_insert(0);
        *cnt += 1;
    }
    println!("{:?}", text_map);

    for key in text_map.keys() {
    println!("{key}");
}
}
