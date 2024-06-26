use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rand::{Rng, thread_rng};
use crate::errors::BankErrors;

#[derive(Debug, Clone)]
pub struct BankState {
    pub name: String,
    users: HashMap<u32, User>,
    transactions: Vec<Transaction>,

    //Fields for user's input
    pub input_fio: String,
    pub input_phone: String,
    pub input_money: String,
    pub input_card_num: String

}
impl BankState {
    pub fn new(bank_name: String) -> Self {
        Self {
            name: bank_name,
            users: HashMap::new(),
            transactions: Vec::new(),
            input_fio: String::new(),
            input_phone: String::new(),
            input_money: String::new(),
            input_card_num: String::new()
        }
    }
    pub fn get_users(&self) -> &HashMap<u32, User> {
        &self.users
    }
    pub fn reset_inputs(&mut self) {
        self.input_phone.clear();
        self.input_fio.clear();
        self.input_money.clear();
    }
    pub fn create_user(&mut self, fio: String, phone_number: String, money_in_account: String) -> Result<(), BankErrors> {

        let mut new_card_number = gen_card_number();

        while self.users.contains_key(&new_card_number) {
            new_card_number = gen_card_number();
        }

        let amount:Result<usize, _> = money_in_account.parse();

        let correct_amount;

        match amount {
            Ok(result) => {
                correct_amount = result
            }

            Err(e) => {
                return Err(BankErrors::InvalidAmountProblem)
            }
        }

        let new_user = User {
            fio,
            card_number: new_card_number,
            phone_number,
            money_amount: correct_amount,
        };

        self.users.insert(new_card_number, new_user);
        Ok(())
    }

    pub fn delete_user(&mut self, card_number: String) -> Result<(), BankErrors> {
        let card = convert_card_num(card_number);
        match self.users.remove_entry(&card) {
            Some((_,_)) => Ok(()),
            None => Err(BankErrors::CantFindUserByCard(format!("Cant find user with card number {card}")))
        }
    }

    pub fn transfer_by_phone(&mut self, sender_card: String, recipient_phone: String, amount: String) -> Transaction {

        let correct_amount = convert_amount(amount);
        let converted_sen_card = convert_card_num(sender_card);
        {
            match self.users.get_mut(&converted_sen_card) {
                Some(user) => {
                    if user.is_enough_money(correct_amount) { user.refuse_money(correct_amount) } else { panic!("Not enough money for transfer!") }
                },
                None => panic!("Sender was not found!")
            }
        }

        match self.users.values_mut().find(|user| user.phone_number == recipient_phone) {
            Some(user) => {
                user.receive_money(correct_amount);
                return Transaction::new(correct_amount, converted_sen_card, recipient_phone.parse::<usize>().unwrap())
            },
            None => panic!("Recipient was not found!")
        }

    }

    pub fn transfer_by_card(&mut self, sender_card: String, recipient_card: String, amount: String) -> Transaction {

        let correct_amount = convert_amount(amount);
        let converted_sen_card = convert_card_num(sender_card);

        {
            match self.users.get_mut(&converted_sen_card) {
                Some(user) => {
                    if user.is_enough_money(correct_amount) { user.refuse_money(correct_amount) } else { panic!("Not enough money for transfer!") }
                },
                None => panic!("Sender was not found!")
            }
        }

        let converted_rec_card = convert_card_num(recipient_card);


        match self.users.get_mut(&converted_rec_card) {
            Some(user) => {
                user.receive_money(correct_amount);
                return Transaction::new(correct_amount, converted_sen_card, converted_rec_card as usize)
            },
            None => panic!("Recipient was not found!")
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push(tx)
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

#[derive(Debug, Clone)]
pub struct User {
    fio: String,
    card_number: u32,
    phone_number: String,
    money_amount: usize,
}
impl User {

}
impl BankUser for User {
    fn check_fio(&self) -> &str {
        &self.fio
    }
    fn check_phone(&self) -> &str { &self.phone_number}
    fn check_card_number(&self) -> u32 {
        self.card_number
    }
    fn check_balance(&self) -> usize {
        self.money_amount
    }
    fn is_enough_money(&self, amount: usize) -> bool {
        if self.money_amount >= amount {true} else {false}
    }
    fn receive_money(&mut self, amount: usize) {
        self.money_amount += amount;
    }
    fn refuse_money(&mut self, amount: usize) {
        self.money_amount -= amount;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Transaction {
    time_of_creation: DateTime<Utc>,
    amount: usize,
    sender_card: u32,
    recipient: usize
}
impl Transaction {
    fn new(amount: usize, sender: u32, recipient: usize) -> Self {
        Self {
            time_of_creation: Utc::now(),
            amount,
            sender_card: sender,
            recipient
        }
    }
    pub fn get_tx_time(&self) -> DateTime<Utc> {
        self.time_of_creation
    }
    pub fn get_amount(&self) -> usize {
        self.amount
    }
    pub fn get_sender_card(&self) -> u32 {
        self.sender_card
    }
    pub fn get_recipient(&self) -> usize {
        self.recipient
    }
}

fn gen_card_number() -> u32 {
    thread_rng().gen_range(1000_0000..9999_9999)
}

fn convert_amount(amount: String) -> usize {
    let correct_amount = match amount.parse::<usize>() {
        Ok(amount) => amount,
        Err(error) => panic!("Can't be parsed!.Incorrect amount input")
    };
    correct_amount
}

fn convert_card_num(card_num: String) -> u32 {
    let correct_card = match card_num.parse::<u32>() {
        Ok(card) => card,
        Err(error) => panic!("Can't be parsed!.Incorrect card number input")
    };
    correct_card
}

pub trait BankUser {
    fn check_fio(&self) -> &str;
    fn check_phone(&self) -> &str;
    fn check_card_number(&self) -> u32;
    fn check_balance(&self) -> usize;
    fn is_enough_money(&self, amount: usize) -> bool;
    fn receive_money(&mut self, amount: usize);
    fn refuse_money(&mut self, amount: usize);

}

