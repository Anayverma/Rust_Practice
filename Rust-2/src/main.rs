fn test(st:String) -> String {
    println!("{}",st);
    return st
}

fn main(){
    let  ans= String::from("anay verma");
    println!("Yes {}",test(ans));
}