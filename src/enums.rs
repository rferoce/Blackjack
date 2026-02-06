#[derive(Debug)]
pub enum FinalScore {
    UserWins,
    Draw,
    DealerWins,
}

impl PartialEq for FinalScore {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FinalScore::UserWins, FinalScore::UserWins) => true,
            (FinalScore::Draw, FinalScore::Draw) => true,
            (FinalScore::DealerWins, FinalScore::DealerWins) => true,
            _ => false,
        }
    }
}