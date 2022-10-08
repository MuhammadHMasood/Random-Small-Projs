use std::iter::{zip, Zip};
use std::collections::HashMap;
use rand::Rng;

const DLIST: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

macro_rules! add_vec{
       ($a:expr,$b:expr)=>{
           {
            let u: [i32; 2] = $a;
            let v: [i32; 2] = $b;
            let t: [i32; 2] = zip(u, v).map(|(x, y)| x+y).collect::<Vec<i32>>()
            .try_into()
            .unwrap();
            t
           }
       }
   }

fn main() {
    //println!("Hello, world!");
    //let mut x: [i32; 2] = [1, 7];
    //let mut y: [i32; 2] = [2, 2];
    //let z: Vec<i32> = zip(x, y).map(|(x, y)| x+y).collect();
    //let z: [i32; 2] = add_vec!(x, y);    
    //print!("{:?}", z);
    //assert_eq!([0, 0], x)
    let mut david: DrunkMan = DrunkMan::new();
    //let mut i = 0;
    while (!david.random_walk())  {
        //println!("{:?}", david.pos);
        //i += 1;
        //if (i > 100) {break}
    }
    println!("{:?}",david.pos_history)
}

struct DrunkMan {
    pos: [i32; 2],
    pos_history: HashMap<Direction, u32>,
    origin: [i32; 2]
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn d2c(dir: Direction) -> [i32; 2] {
    let x = match dir {
        Direction::Up => [0, 1],
        Direction::Down => [0, -1],
        Direction::Left => [-1, 0],
        Direction::Right => [1, 0]
    };
    //println!("Direction: {:?}", x);
    x
}

fn add_vec(z: Zip<[i32; 2], [i32; 2]>) -> Vec<i32> {
    return Vec::new();
}


impl DrunkMan {
    fn new() -> DrunkMan {
        let mut x = DrunkMan {
            pos: [3, 3],
            pos_history: HashMap::new(),
            origin: [0, 0]
        };
        x.pos_history.insert(Direction::Up, 0);
        x.pos_history.insert(Direction::Down, 0);
        x.pos_history.insert(Direction::Left, 0);
        x.pos_history.insert(Direction::Right, 0);
        return x;
    }
    fn random_walk(&mut self) -> bool {

        let transform = DLIST[rand::thread_rng().gen_range(0..4)];
        self.pos_history.insert(transform, self.pos_history.get(&transform).ok_or_else(|| 0).unwrap() + 1);
        self.pos = add_vec!(self.pos,d2c(transform));

        //zip(self.pos,d2c(DLIST[rand::thread_rng().gen_range(0..4)]))
        //    .map(|(x,y)| x+y)
        //    .collect();

        if self.pos == self.origin {
            return true;
        }
        return false;
    }
}