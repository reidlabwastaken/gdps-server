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
    pub fn to_star_difficulty(&self) -> i32 {
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
}