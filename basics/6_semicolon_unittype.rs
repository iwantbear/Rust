// () - empty tuple, unit type (void)
// empty_tuple 변수에 아무것도 없으면 빈 튜플 반환 
// 따라서 변수 불러와도 빈 튜플 가져옴


fn empty_tuple() {

}


// Display print = {}
// Debug print = {:?}
// Devug print는 변수에 뭐가 들어있는지 확인. 즉, 개발자가 보고싶을때 
fn main() {
    let tuple = empty_tuple;
    tuple;
    6
}

fn main() {
    let tuple = empty_tuple();
    println!("{}", tuple)
}
