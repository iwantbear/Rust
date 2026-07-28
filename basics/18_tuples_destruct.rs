// tuples
// array와 다르게 다른 타입을 같이 담을 수 있음

fn main() {
    let my_tuples = (8, "David MacLoed", vec![0, 1, 2]);
    println!("{:?}", my_tuples);
}


// tuples 두개를 담고 있는 vec
fn main() {
    let my_vec = vec![("Hey", 9), ("Hello", 124)];
    println!("{:?}", my_vec);
}



// 값을 꺼낼 때는 .번호 
// 여기에서 번호는 튜플에 담겨있는 인덱스임 밑의 코드에서 random_tuple.0 같은것
fn main() {
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    )
}



// 튜플의 값을 꺼낼때 위 방법처럼 하나씩 꺼내는게 아닌 전부를 받아서 원하는것만 꺼냄
// 오류는 안나오지만 b, c 를 사용하지 않았기 때문에 경고가 나옴
fn main() {
    let str_tuple = ("one", "two", "three");
    let (a, b, c) = str_tuple;
    println!("Item a is : {}", a);
}


// b, c 안쓰고 싶으면 밑에처럼 하면됨
fn main() {
    let str_tuple = ("one", "two", "three");
    let (a, _, _) = str_tuple;
    println!("Item a is : {}", a);
}