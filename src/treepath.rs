pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub enum TOS {
    Leaf(String),
    Branch(Box<TOS>, Box<TOS>),
}

pub fn tree_pick(tos: TOS, lod: Vec<Direction>) -> Option<TOS> {
    lod.iter().fold(
        Some(tos),
        |here: Option<TOS>, direction: &Direction| match here {
            None => None,
            Some(tos) => pick_direction(tos, direction),
        },
    )
}

pub fn pick_direction(tos: TOS, direction: &Direction) -> Option<TOS> {
    match (direction, tos) {
        (Direction::Left, TOS::Branch(l, _)) => Some(*l),
        (Direction::Right, TOS::Branch(_, r)) => Some(*r),
        (Direction::Left, TOS::Leaf(_)) => None,
        (Direction::Right, TOS::Leaf(_)) => None,
    }
}

pub fn test_tree() -> TOS {
    TOS::Branch(
        Box::new(TOS::Leaf("a".to_string())),
        Box::new(TOS::Leaf("b".to_string())),
    )
}

pub fn test_directions() -> Vec<Direction> {
    vec![Direction::Left]
}
