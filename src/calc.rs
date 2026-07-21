//basically calling its self
fn fib(n:i32)->i32 {
//variable already declared in f'n definition
if n < 2 {
return n;

}else{return fib(n-1)+fib(n-2);
}

}

fn main(){
  let n=13;
  let w=String::from("who is louis");
 fib(n);
//c=1+2   v=c+2    b=c+v
println!("{}",fib(n));

println!("Tracy typed {}",w)
}