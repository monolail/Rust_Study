// #[test]
// fn iter_test(){
//     let v = vec![1,2,3,4,5];
//     for val in v.iter() {  //v.iter()를 사용했다.
//         print!("{} ", val); // 1 2 3 4 5 
//     }

//     println!("");

//     for val in &v {   //for 루프에서는 벡터의 iterator가 자동으로 나온다. 
//         print!("{} ", val); // 1 2 3 4 5 
//     }
// }

// #[test]
// fn into_iter_test(){
//     let v = vec![1,2,3,4,5];
//     for val in v.into_iter() {  //into_iter()를 사용했다. 소유권이 넘어간다.
//         print!("{} ", val); // 1 2 3 4 5 
//     }

//     println!("");

//     // for val in &v {   //여기서 에러 발생한다. 위에서 v.into_iter()에 의해 소유권이 넘어갔기 때문.
//     //     print!("{} ", val); // 1 2 3 4 5 
//     // }
// }

// #[test]
// fn iter_mut_test(){
//     let mut v = vec![1,2,3,4,5];
//     for x in v.iter_mut(){
//         *x += 1;   //borrow한 것이기에 *를 붙여야 된다.
//     } 
//     println!("{:?}",v);   // [2, 3, 4, 5, 6]

//     for val in &v {   //동작 가능하다. iter_mut()에 의해 소유권이 넘어가지 않기 때문
//         print!("{} ", val); // 2 3 4 5 6 
//     }
// }

// #[test]
// fn iter_mut_for_each_test(){
//     let mut v = vec![1,2,3,4,5];  
//     v.iter_mut().for_each(|x| *x += 1);     
//     println!("{:?}",v);

//     for val in &v {
//         print!("{} ", val); //  2 3 4 5 6 
//     }
// }


fn main(){
    // 1. 각 원소에 대해 2를 곱해서 다른 벡터로 만듦
    let v = vec![1,2,3,4,5];
    let v1:Vec<_> = v.iter().map(|x| (*x)*2).collect();
    println!("{:?}",v1);  //[2, 4, 6, 8, 10]

    // map(|x| x*2): 이렇게 해도 됨. 간단.
    let v2:Vec<_> = v.iter().map(|x| x*2).collect();  
    println!("{:?}",v2); //[2, 4, 6, 8, 10]

    // into_iter()를 써도 됨
    let v = vec![1,2,3,4,5];
    let v1:Vec<_> = v.into_iter().map(|x| x*2).collect();
    println!("{:?}",v1);  //[2, 4, 6, 8, 10]

    // collect::<Vec<u32>>()와 같이 해도 됨
    let v = vec![1,2,3,4,5];
    let v1 = v.iter().map(|x| (*x)*2).collect::<Vec<u32>>();
    println!("{:?}",v1);  //[2, 4, 6, 8, 10]

    //2. 문자열을 모두 소문자로 변환
    let words:Vec<&str> = vec!["Hello", "Good Morning", "Hi"];
    let low_words:Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();
    println!("{:?}",low_words);  //["hello", "good morning", "hi"]        

    //3. 문자열에서, 각 문자에 대해 문자열에서 해당 문자의 개수를 HashMap으로 저장. (ch: cnt)
    use std::collections::HashMap;
    let s = "abc aaa bb c";
    let map = "abc".chars()
        .map(|c| (c,s.matches(c).count()))
        .collect::<HashMap<char,usize>>();
    println!("{:?}",map);  //{'a': 4, 'c': 2, 'b': 3}

    // (참조)어떤 문자열에서 알파벳만을 집합으로 뽑아낼 때
    use std::collections::HashSet;
    let s = "abc aaa bb c";
    let set = s.chars()
        .filter(|c| c.is_alphabetic())        
        .collect::<HashSet<_>>();        
    println!("{:?}",set);  //{'c', 'a', 'b'}
}
