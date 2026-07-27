// Ownership in copy types
// 숫자 (i32, f64)나 bool, char는 &나 &mut을 안붙여도 소유권이 안넘겨지고 복사
// string은 복사가 안됨 소유권 넘어감

// copy - copy types
// clone - non copy types

// 밑의 코드에서 my_number 변수는 정수 (i32) 라서 소유권 이동이 아닌 복사
// 하지만 my_country는 String 타입이라 복사가 아닌 소유권 이동이라 오류
// 오류가 나는 이유는 &를 붙이지 않음 
// 오류가 안나려면 prints_string 함수의 String 앞과 
// main 함수의 prints_string(my_country)의 my_country 앞에 붙이면됨
// 또 오류가 안나려면 .clone() 쓰면됨 문제는 메모리를 거의 두배 먹음, 필요할때만...

fn prints_number(number: i32) {
    println!("{number}");
}

fn prints_string(input: String) {
    println!("{input}");
}

fn main() {
    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let my_country = "호주".to_string();
    prints_string(my_country.clone()); // 복사본을 함수로 넘김
    prints_string(my_country);
}