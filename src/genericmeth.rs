fn gen<T>(n:i32,even:T,odd:T)->T{//generic method returns the genric type T
if n%2==0 {even}else {odd}
}
fn main(){

    println!("{:?}",gen(11,4,7));
}