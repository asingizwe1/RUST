//allowing you declare variable with same name as previous
//shadowing allows you to reuse same name without mutability
fn main(){
    let a = 11;
    println!("outer:{}",a);
{
    let a ="louis";
    println!("inner:{}",a);
}


}


