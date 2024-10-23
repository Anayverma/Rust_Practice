fn main() {
    println!("Hello, world!");
    println!("Fibonacci number is - {}",fibonacci(560));
}


// function to calculate fib
fn fibonacci(mut n :u64)->u64{
    let mut a:u64=0;
    let mut b=1;
     

    if n < 2 {return n;}
    else 
    {
        n=n-2;
        let mut c=1;
        while n > 0 {
            c= a+b;
            n-=1;
            a=b;
            b=c;
        }
        return c;
    }
}
