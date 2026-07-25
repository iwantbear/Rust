// Ownership - 소유권
// 현재 "대한민국" 소유권은 country 변수
// &붙이고 country를 불러오면 참조 가능

fn main() {
    let country = "대한민국".to_string();
    let ref_one = &country;
    let ref_two = &country;
    println!("Country is: {}", ref_one);
}