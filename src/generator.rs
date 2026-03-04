pub struct Puzzle {
    size: u8,
    array: Vec<Vec<u8>>
}

impl Puzzle {
    pub fn init(size: u8) -> Self {
        let arr = setup();
        Puzzle { size, array: arr}
    }
}

pub fn setup() -> Vec<Vec<u8>>{
    
}
