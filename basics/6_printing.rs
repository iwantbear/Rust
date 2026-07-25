// ln 없으면 한줄로 나옴 (ThisThis)

fn main() {
    print!("This");
    print!("This");
}


// \n은 줄바꿈
fn main() {
    print!("My\nname\nis\nhan");
}


// r##: raw text 즉, 경로 같은거 프린트문으로 볼때 쓰면 좋음 
// (뒤에 new_drive 부분에서 \n을 줄바꿈으로 인식 하는 문제가 생길 수 있음)
fn main() {
    print!(r#"c:\thisdrive\new_drive"#);
}


// 줄바꿈을 알아서 해주긴 하는데 들여쓰기가 잘못되었을 수 있음
// 밑의 경우에서 실제로 실행하면 다음과 같이 뜸 
// Let me tell you
//     어떤 이야기를 
//     봅시다

fn main() {
    println!("Let me tell you
    어떤 이야기를 
    봅시다");
}


// 다음과 같이 앞의 부분을 당겨 쓰면 각 줄의 맨 앞에 부터 글이 나옴
// 밑의 경우에서 실제로 실행하면 다음과 같이 뜸 
// Let me tell you
// 어떤 이야기를
// 봅시다

fn main() {
    println!(
"Let me tell you
어떤 이야기를 
봅시다");
}


// Debug print = println!("{:?}") 출력되지 않는 부분을 개발자만 볼 수 있게, 다음장에서 자세히
// Pointer print = println!("{:p}") 포인터 즉, 메모리 위치 프린트
// Hexadecimal print (16진수) = println!("{:x}") 16진수로 프린트
// Byte print = println!("{:b}") 바이트로 프린트 
fn main() {
    let my_variable = &9;
    println!("{:p}", my_variable);   // 결과 = 0x100446af0
}

fn main() {
    let my_variable2 = 9000;
    println!("{:x}", my_variable2);  // 결과 = 2328
}

fn main() {
    let my_variable3 = 4124;
    println!("{:b}", my_variable3);  // 결과 = 1000000011100
}



// -^30 = 전체 길이 30자 고정, ^는 텍스트를 가운데로 정렬, -는 나머지를 -로 채움
// < = 왼쪽 정렬, > = 오른쪽 정렬
// a:-<15는 변수 a를 왼쪽 정렬, 15길이 중 남은걸 왼쪽에서 -로 채움
fn main() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{a:-<15}{b:->15}"); // variable names city1 and city2, pad with -, one to the left, one to the right
}
