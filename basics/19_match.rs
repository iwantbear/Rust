// Control fow

// 밑의 코드처럼 써도 동작은 하지만 rust는 더 편하게 쓸 수 있음
fn main() {
    let my_number = 5;

    // rust 에서는 if (my_number == 7) 처럼 안써도 됨 즉, ()로 안 감싸도됨
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six");
    } else {
        println!("It's a different number");
    }
}



// match는 러스트에서 if-else 문을 더 쉽게 쓸 수 있는 문법임
// => 를 fat arraow 라고 함 "왼쪽일 경우에 오른쪽 처럼 실행하라"

// 근데 이렇게 코드 쓰면 오류가 남 
// 변수값은 5인데 0일때와 1일때의 경우만 존재하기 때문 
// my_number가 u8 이므로 0부터 255인데 두가지 경우 말고 나머지 경우가 없음
fn main() {
    let my_number: u8 = 5;

    match my_number {
        0 => println!("It's a zero"),
        1 => println!("It's a one"),
    }
}


// 돌아가게 하려면 _ 추가하면됨
// tuple 섹션에서 봤듯이 _는 나머지 경우 _는 보통 마지막에 씀 
// _를 첫줄에 써도 오류는 안나지만 보통 그렇게 안함 
fn main() {
    let my_number: u8 = 5;

    match my_number {
        0 => println!("It's a zero"),
        1 => println!("It's a one"),
        _ => println!("It's a different number")
    }
}


// 이건 match가 값을 second_number 변수에 돌려줘서 출력함
// 즉, my_number가 5이니까 _ 경우에 해당하는 값 0이 second_number에 담기고 출력
fn main() {
    let my_number: u8 = 5;
    
    let second_number = match my_number {
        0 => 23,
        1 => 65,
        _ => 0
    };
    println!("The second number is : {}", second_number);
}



// 변수가 두개일때 match 사용하는 법
// 이 코드는 데이터 값을 match함
fn main() {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),

        // _ 표시는 아무거나 상관없기에 이 줄의 프린트문이 나올것
        ("cloudy", _) => println!("Cloudy and somthing else."),
        _ => println!("Not sure what the weather is.")
    }
}


//이 코드는 변수 이름을 match 하면서 if 문 예제
fn main() {
    let children = 5;
    let married = true;

    match (children, married) {

        (children, married) if married == false => println!("Not married with {} children", children),
        // (c, m)으로 줄여서 써도 컴파일됨
        (c, m) if c == 0 && m => println!("Married but with no children"),
        _ => println!("Some other type of marriage and children combination")
    }
}




// match는 한 조건씩 봄 first 변수로 예를 들어 설명해보면
// first는 (200, 0, 0)임 
// 이때 200은 첫줄의 r < 10을 만족못하니까 두번째 줄로 내려감
// 0은 두번째 줄의 g < 10을 만족하니까 여기에서의 프린트 문이 출력됨
// 나머지 변수들도 한줄씩 내려가며 조건을 확인하고 출력함
fn match_colours(rgb: (u32, u32, u32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Every clour has at least 10")
    }
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}




// 다음 코드는 오류남
// match가 반환할때 타입이 같아야 함 my_number = i32, _ = &str
fn main() {
    let my_number = 10;
    let come_variable = match my_number {
        10 => 8,
        _ => "Not ten"
    };
}



// match에서 범위안에 걸린 값을 꺼내 쓰고 싶을땐 @ 사용하면 됨
fn match_number(input: i32){
    match input {
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {number}"),
        _ => println!("It's greater than ten")
    }
}

fn main() {
    match_number(10);
    match_number(100);
}