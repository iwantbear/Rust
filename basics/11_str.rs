// 두가지 종류 있음 
// String (더 편리, 더 기능이 많음) = 가변, 소유권 있음 
// &str / ref str = 불변, 참조만 

// 문자열 리터럴은 수정 불가능, 프로그램 바이너리에 박혀있음
// String은 수정 가능, 힙 메모리에 저장


fn main() {
    // 밑의 두개는 똑같이 문자열 리터럴을 String으로 변환
    let my_name = "David".to_string();  
    let other_name = String::from("David2");

    // growable + shrinkable
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');    // !는 문자이기 때문에 ''를 써야함
    println!("{}", my_other_name);
}




// .push()는 문자 하나 추가 
// .push_str()는 문자열 추가
// .capacity는 용량을 늘려줌 쉽게 말해 변수가 늘어날것을 예상해 메모리 용량을 늘림
// 변수만 용량을 늘림, 첫 변수의 문자열이 비어있으면 최소 바이트인 8 할당

// 밑에서 David는 5바이트고 !가 하나 추가됐으니 6으로 메모리가 딸림 
// 이때 기존 변수에 두배를 늘려 메모리 크기는 10이 됨 
// 근데 !가 세개 늘어났지만 그래도 9바이트 이므로 더 늘리진 않음
fn main() {
    let mut my_name = "David".to_string();

    my_name.push('!');
    my_name.push_str("!!!");

    println!("My name is {}", my_name);
    println!("My name is {}", my_name.capacity());
}


// 처음부터 메모리 용량 지정가능
fn main() {
    let mut s = String::with_capacity(20);
    s.push_str("");
    s.push_str("Gyu Han");
    println!("My name is {} {}", s, s.capacity());
}