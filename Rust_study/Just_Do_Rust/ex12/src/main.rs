#[test]
fn filter_test(){
    let v = vec![1,2,3,4,5];
    let v1:Vec<_> = v.iter().filter(|&&x| x%2==0).collect();

    println!("v={:?}",v);  //v=[1, 2, 3, 4, 5]
    println!("v1={:?}",v1);  //v1=[2, 4]
}

#[test]
fn filter_test1(){
    let v = vec![1,2,3,4,5];
    let v1:Vec<_> = v.iter().filter(|x| *x%2==0).collect();

    println!("v={:?}",v);  //v=[1, 2, 3, 4, 5]
    println!("v1={:?}",v1);  //v1=[2, 4]
}