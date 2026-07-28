// From and Into

fn main() {

    // "Dave Macleod"는 &str임
    // From은 &str로 부터 String을 만듬 
    // 즉, 네가 나한테와 
    let my_name = String::from("Dave Macleod");

    // &str을 String으로 바꿔줘 
    // 즉, 내가 너한테 갈게
    let my_city: String = "Seoul".into();

}