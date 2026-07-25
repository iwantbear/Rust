// string의 경우 무조건 ""('' 안됨)
// 하지만 char의 경우 ''
// 모든 char는 4bytes
fn main() {
    println!("Hello, world!");
    let first_letter = 'A';
    let space = ' '; // A spcae inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too
}

// casting (캐스팅)
// casting = simple type change using 'as'
// 즉, integer 장에서 말한 다른 타입의 덧셈이 안될때 as 사용하면 가능
fn main() {
    println!("Hello world!");
    let my_number: u16 = 8; 
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16;
    println!("{}", third_number)
}


// ASCII CODE
// 밑의 my_number에 할당된 a는 char임 
// casting 사용하면 a에 해당하는 아스키 코드는 97
// 별로 쓸 일 없을듯? 있나?
fn main() {
    let my_number = 'a' as u8;
    println!("My number is : {}", my_number)
}



// .len() 함수는 바이트를 말함
// .chars().count() 함수는 글자 수를 셈
fn main() {
    // 프린트 문에 변수를 사용하려면 {} 꼭 써야함
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}