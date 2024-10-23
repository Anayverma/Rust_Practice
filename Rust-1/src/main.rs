fn main(){
    let ans = check(59);
    println!("hello ji  {}",ans);
    println!("hi");
}
fn check(num: u32) -> bool {
    if num%2==0 {
        return true;
    }
    else {
        return false;
    }
}