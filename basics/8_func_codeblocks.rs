// 변수에 숫자 넣을때 타입 꼭 써야함
// 세미콜론 없음 : 그거 반환
// 세미콜론 있음 : 해당 줄 실행하고 끝

// 기본 1 : 함수가 값을 반환 → 밖에서 사용 가능
fn give_number(one: i32, two: i32) -> i32 {
    one * two
}

fn main() {
    let my_number = give_number(9, 8);
    println!("{}", my_number);
}




// 기본 2 : 함수가 값을 반환하지 않고 내부에서 바로 출력
fn print_number(one: i16, two: i16) {
    let multiplied = one * two;
    println!("{}", multiplied);
}

fn main() {
    print_number(7, 8);
}




// 기본 3  
fn print_number(one: i16, two: i16) -> i16 {
    let multiplied_by_ten = {
        let first_number = 10;     // 이건 함수 안에서만 10으로 사용가능
        first_number * one * two
    };
    multiplied_by_ten
}

fn main() {
    let my_number = print_number (7, 8);
    println!("{}", my_number);
}

//------------------------------------------------------------------
// Ownership을 function에 접목
// 자세한 Ownership은 다음장
// 밑의 코드는 아무 문제 없음

fn print_country(country_name: String) {
    println!("My country is {country_name}");
}

fn main() {
    let country = "대한민국".to_string();
    print_country(country);
}



// 위 코드에서 print_country(country); 한줄만 추가하면 오류가남
// 밑의 경우를 move semantics 라고함
// 첫번째 print_country(country); 에서 "대한민국" 이라는 데이터는 country가 소유
// 근데 이걸 print_country로 넘겨줌으로써 소유권이 country_name으로 변경됨
// 그리고 함수가 끝나기에 contry_name 자체도 사라지고 데이터(대한민국)도 사라짐
// print_country(&country); 로 쓰면 빌려주는 것이기에 오류 안남

fn print_country(country_name: String) {
    println!("My country is {country_name}");
}

fn main() {
    let country = "대한민국".to_string();
    print_country(country);
    print_country(country);
}


// mut를 function에 접목
// 자세한 mut 내용은 다음장에

// main 함수에서 수정가능한 문자열 타입인 my_country를 정의함 (캐나다로)
// &mut를 붙여서 수정가능하게 add_is_great 함수로 넘김
// my_country 라는 변수 이름을 자연스럽게 country_name으로 바꿈
// rust는 넘길때, 호출할때 전부 변경할 수 있는 기준 쌍을 맞춰야 함
// 즉, 넘길때 &mut 였으면 호출할때도 &mut를 명시해야함 
// country_name이 캐나다인데 뒤에 is great!를 붙임
// 다시 main으로 돌아가서 add_is_great 한번더 호출

// 참고로 .push_str 같은 매서드를 사용하면 *(역참조)를 알아서 수행해줌

fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great!");
    println!("Now it says: {country_name}");
}

fn main() {
    let mut my_country = "캐나다".to_string();
    add_is_great(&mut my_country);
    add_is_great(&mut my_country);
}



// 위 코드랑 다른 유일한 차이점은 이 코드에선 소유권을 아예 넘겨버림
// main 함수 안에선 my_country가 소유권을 갖고 있는데 
// 두번째 줄의 add_is_great(my_country); 이 부분에서 (my_country) 앞에
// &나 &mut가 없기 때문에 아예 소유권을 넘겨버린것

fn add_is_great(mut country_name: String) {
    country_name.push_str(" is great!");
    println!("Now it says: {country_name}");
}

fn main() {
    let mut my_country = "대한민국".to_string();
    add_is_great(my_country);
}

// 바로 위 두 코드의 차이점이 하나더 있는데 
// 위 코드의 main 함수에서 add_is_great 함수 호출할때 &mut를 () 안에 넣었으면
// 불러오는 함수의 String 앞에 붙이면됨

// 밑의 코드처럼 main 함수에서 add_is_great 함수 호출할때 ()안에 아무것도 없으면
// 불러오는 함수의 변수 앞에 mut 붙이면됨 