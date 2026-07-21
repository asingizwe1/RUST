fn main(){
//if let- doesnt support guard clauses-handling single pattern
let val = Some(5);


//PATTERNS USING SOME AND NONE


if let Some(6)= val {
    println!("");
}else{ println!("none");}


//lets u execute code depending on whther code matches pattern
//let else
//while let- loop will go through as long as value matches pattern
let mut s = vec![1,2,3,4];
while let Some(top)=s.pop(){
    println!("{}",top);
}

}