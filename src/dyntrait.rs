trait Animal{fn sound(&self);}
struct Dog;
impl Animal for Dog{
    fn sound(&self){println!("woof");}
}

fn makeSound(key:&dyn Animal){//..self is only available for functions with self parameter
key.sound();//this function cant have a self parameter
}
fn main(){
let dog = Dog;
makeSound(&dog);
}