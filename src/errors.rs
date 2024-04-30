pub enum BankErrors {
    InvalidAmountProblem,
    TransferProblem,
    CantFindUserByCard(String),
}