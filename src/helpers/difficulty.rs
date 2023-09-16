#[derive(PartialEq, Copy, Clone)]
pub enum LevelDifficulty {
    Auto,
    Easy,
    Normal,
    Hard,
    Harder,
    Insane,
    Demon
}

impl LevelDifficulty {
    pub fn new(value: i32) -> LevelDifficulty {
        match value {
            0 => LevelDifficulty::Auto,
            1 => LevelDifficulty::Easy,
            2 => LevelDifficulty::Normal,
            3 => LevelDifficulty::Hard,
            4 => LevelDifficulty::Harder,
            5 => LevelDifficulty::Insane,
            6 => LevelDifficulty::Demon,
            _ => panic!("invalid level difficulty")
        }
    }

    pub fn value(self) -> i32 {
        match self {
            LevelDifficulty::Auto => 0,
            LevelDifficulty::Easy => 1,
            LevelDifficulty::Normal => 2,
            LevelDifficulty::Hard => 3,
            LevelDifficulty::Harder => 4,
            LevelDifficulty::Insane => 5,
            LevelDifficulty::Demon => 6,
        }
    }

    pub fn to_star_difficulty(self) -> i32 {
        match self {
            LevelDifficulty::Auto => 5,
            LevelDifficulty::Easy => 1,
            LevelDifficulty::Normal => 2,
            LevelDifficulty::Hard => 3,
            LevelDifficulty::Harder => 4,
            LevelDifficulty::Insane => 5,
            LevelDifficulty::Demon => 5,
        }
    }

    pub fn stars_to_diff(stars: i32) -> Self {
        match stars {
            1 => LevelDifficulty::Auto,
            2 => LevelDifficulty::Easy,
            3 => LevelDifficulty::Normal,
            4 | 5 => LevelDifficulty::Hard,
            6 | 7 => LevelDifficulty::Harder,
            8 | 9 => LevelDifficulty::Insane,
            10 => LevelDifficulty::Demon,
            _ => panic!("invalid difficulty!")
        }
    }
}

pub enum DemonDifficulty {
    Easy,
    Medium,
    Hard,
    Insane,
    Extreme
}

impl DemonDifficulty {
    pub fn new(value: i32) -> DemonDifficulty {
        match value {
            0 => DemonDifficulty::Easy,
            1 => DemonDifficulty::Medium,
            2 => DemonDifficulty::Hard,
            3 => DemonDifficulty::Insane,
            4 => DemonDifficulty::Extreme,
            _ => panic!("invalid demon difficulty")
        }
    }

    pub fn value(self) -> i32 {
        match self {
            DemonDifficulty::Easy => 0,
            DemonDifficulty::Medium => 1,
            DemonDifficulty::Hard => 2,
            DemonDifficulty::Insane => 3,
            DemonDifficulty::Extreme => 4
        }
    }

    pub fn to_demon_difficulty(self) -> i32 {
        match self {
            DemonDifficulty::Easy => 3,
            DemonDifficulty::Medium => 4,
            DemonDifficulty::Hard => 0,
            DemonDifficulty::Insane => 5,
            DemonDifficulty::Extreme => 6
        }
    }
}