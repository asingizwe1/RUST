fn main(){
    //look at m outside for loop scope
    let mut m =0;
    //look at _i
for _i in 0..11 {  
 
  m+=1;
  println!("{}",m);
}

//we can jst iterate directly the elements themselves instead of array index
let array:[&str;2]=["GU","DXY"];
//each j is a member of array
for j in array.iter() {  
  println!("{}",j);
}
}