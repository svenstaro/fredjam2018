#[derive(Debug)]
pub enum Item {
    Gun { usages: i32 },
}

#[derive(Debug)]
pub struct Player {
    pub health: i32,
    pub items: Vec<Item>,
}
