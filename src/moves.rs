pub struct Move(pub i8, pub i8);

pub const MOVES: [Move; 8] = [        
    Move(1, 2),
    Move(2, 1),
    Move(2, -1),
    Move(1, -2),
    Move(-1, -2),
    Move(-2, -1),
    Move(-2, 1),
    Move(-1, 2)
];
