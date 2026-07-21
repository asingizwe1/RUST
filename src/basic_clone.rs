fn main(){
let y =String::from("iop");
let mut p= y.clone();
//in this case we can use y after assigning it to p..
//which wouldnt happen with p=y -pointer y is copied to p not the whole data 
  println!("{}",p);
  
}