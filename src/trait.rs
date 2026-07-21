trait Animal{
   fn sound(&self)->String; 
}
struct Sheep;
impl Animal for Sheep{
    fn sound(&self)->String{
      
        String::from("HIM MR_GU innit")
      }
}
  
fn main(){
let sheep = Sheep;
println!("{}",sheep.sound());
}
