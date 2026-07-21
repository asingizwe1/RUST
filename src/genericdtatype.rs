struct  Point <T>{
x:T,
y:T
} 

impl <T> Point<T>{
fn coord(&self)->(&T,&T){
    &self.x, &self.y 
}

}