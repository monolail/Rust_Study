fn main() {
    let mut sum = 0;
    
    for i in 1..=100{
        sum += i;
    }

    println!("{}", sum);

    let mut sum2 = 0;
    
    for i in 1..=100{
        sum2 += i;

        if sum2 > 4900 {
            break
        }

    }

    println!("{}", sum2);


    let mut i = 1; 
    loop {
        // break 없으면 무한 반복
        if i > 10 { break; } 
        println!("{}", i);         
        i += 1;
    }

    let mut j = 1; 
    let mut hap = 0;
    loop {
        if j >10{break;}

        for i in 1..=10 {
            hap += i;
        }

        j+=1;
    }

    println!("{}", hap); 

    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        print!("{} ", x);   //5 4 3 2 1 
    }
}
