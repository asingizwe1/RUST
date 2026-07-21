fn main(){
enum Direction
{//variants
North,
East,
West,
South,
}

enum Player{

Move(Direction),//tuple variant


}

let x=Player::Move(Direction::North);


}