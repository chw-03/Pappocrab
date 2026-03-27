use std::vec;
use rand::prelude::*;

pub struct Puzzle {
    array: Vec<Vec<u8>>,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::new()
    }
}
impl Puzzle {
    pub fn new() -> Self {
        let arr = setup();
        Puzzle { array: arr }
    }
}

fn rng_set(nums: &Vec<u8>, rng: &mut ThreadRng) -> Vec<u8>{
    let mut entries = nums.clone();
    entries.shuffle(rng);
    entries
}

pub fn setup() -> Vec<Vec<u8>> {
    let mut rng = rand::rng();
    let nums: Vec<u8> = (1..=9).collect();
    let mut array = (0..9).map(|_| vec![0; 9]).collect::<Vec<_>>();
    
    // fill diagonal 3x3s since they are independent
    for i in 0..3 {
        let mut entries = rng_set(&nums, &mut rng);
        for j in 0..3 {
            for k in 0..3 {
                array[j + 3*i][k + 3*i] = entries.pop().unwrap();
            }
        }
    }

    //fill rest of the matrix with feedback
    for row in array.iter_mut() {
        let entries = rng_set(&nums, &mut rng);
        
    }

    for i in 0..9 {
        for j in 0..9 {
            print!("{}", array[i][j]);
        }
        println!()
    }

    array
}
