use std::fmt::format;
//Model
use crate::bank_model::{BankState, BankUser, User};
//Styles
use crate::styles::{ContainerStyle};
mod bank_model;
mod errors;
mod styles;


//iced crate dependencies
use iced::{Alignment, alignment, Application, Element, Length, Padding, Renderer, Sandbox, Settings, Size, Theme, window};
use iced::alignment::{Horizontal, Vertical};
use iced::theme::{Button};
use iced::widget::{button, text_input, container, column, text, row, Space, Container, Row, Scrollable, Column};
use iced::widget::scrollable::{Direction, Properties};


fn main() -> iced::Result {
    <BankApp as Sandbox>::run(Settings {
        window: window::Settings {
            size: Size::new(1200.0, 800.0),
            resizable: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

//Our State of Bank Application
#[derive(Debug)]
enum BankApp {
    BeforeBankCreating(InitialBankState),
    OnlyBankCreated(BankState),
    BankWithUsers(BankState),
    TransferPage(BankState, TransferPageState)
}
//Initialize our first state of app
impl Default for BankApp {
    fn default() -> Self {
        BankApp::BeforeBankCreating(
            InitialBankState {
                input_value: String::new(),
            }
        )
    }
}

#[derive(Debug, Default)]
struct InitialBankState {
    //widgets
    input_value: String,
}

#[derive(Debug, Clone)]
enum BankMessage {
    //Bank's messages
    BankNameChanged(String),
    CreateBank,
    //User's messages
    UserFioChanged(String),
    UserPhoneNumChanged(String),
    UserMoneyChanged(String),
    UserCardNumChanged(String),
    FirstUserCreate,
    CreateUser,
    DeleteUser,
    //TransferPage's messages
    FromUserChanged(String),
    ToUserPhoneChanged(String),
    ToUserCardChanged(String),
    TransferAmountChanged(String),
    ToTransferPage,
    TransferByCard,
    TransferByPhone,
    ToUserPage,
    ByPhoneMode,
    ByCardMode
}

#[derive(Debug)]
struct TransferPageState {
    sender_card_input: String,
    amount_input: String,
    recipient_card_input: String,
    recipient_phone_input: String,
    transfer_mode: TransferMode
}
impl Default for TransferPageState {
    fn default() -> Self {
        Self {
            sender_card_input: String::new(),
            amount_input: String::new(),
            recipient_card_input: String::new(),
            recipient_phone_input: String::new(),
            transfer_mode: TransferMode::default()
        }
    }
}
impl TransferPageState {
    fn reset_inputs(&mut self ) {
        self.sender_card_input.clear();
        self.amount_input.clear();
        self.recipient_card_input.clear();
        self.recipient_phone_input.clear();
    }
}
#[derive(Debug, Default)]
enum TransferMode {
    #[default]
    TransferByPhone,
    TransferByCard
}

impl Sandbox for BankApp {
    type Message = BankMessage;
    
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Bank".to_string()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }

    fn update(&mut self, message: BankMessage) {
        match self {
            BankApp::BeforeBankCreating(InitialBankState) => {
                match message {

                    BankMessage::BankNameChanged(value) => {
                        InitialBankState.input_value = value;
                    }

                    BankMessage::CreateBank => {
                        let bank = BankState::new(InitialBankState.input_value.clone());
                        *self = BankApp::OnlyBankCreated(bank)
                    }

                    _ => {}
                }
            }

            BankApp::OnlyBankCreated(BankState) => {
                match message {
                    BankMessage::UserFioChanged(fio) => {
                        BankState.input_fio = fio;
                    }

                    BankMessage::UserPhoneNumChanged(phone) => {
                        BankState.input_phone = phone;
                    }

                    BankMessage::UserMoneyChanged(money) => {
                        BankState.input_money = money;
                    }

                    BankMessage::FirstUserCreate => {
                        match BankState.create_user(BankState.input_fio.clone(), BankState.input_phone.clone(), BankState.input_money.clone()) {
                            Ok(()) => {
                                BankState.reset_inputs();
                                *self = BankApp::BankWithUsers(BankState.clone());
                            },
                            Err(e) => println!("Invalid amount was given")
                        }
                    }

                    _ => {}
                }
            }

            BankApp::BankWithUsers(BankState) => {
                match message {
                    BankMessage::TransferByPhone => {
                        todo!()
                    }

                    BankMessage::UserFioChanged(fio) => {
                        BankState.input_fio = fio;
                    }

                    BankMessage::UserPhoneNumChanged(phone) => {
                        BankState.input_phone = phone;
                    }

                    BankMessage::UserMoneyChanged(money) => {
                        BankState.input_money = money;
                    }

                    BankMessage::UserCardNumChanged(card) => {
                        BankState.input_card_num = card;
                    }

                    BankMessage::CreateUser => {
                        let new_user = BankState.create_user(BankState.input_fio.clone(), BankState.input_phone.clone(), BankState.input_money.clone());
                        BankState.reset_inputs();
                    }

                    BankMessage::DeleteUser => {
                        match BankState.delete_user(BankState.input_card_num.clone()) {
                            Ok(()) => {},
                            Err(e) => {
                                panic!("Cant find particular user")
                            }
                        }
                    }

                    BankMessage::ToTransferPage => {
                        *self = BankApp::TransferPage(BankState.clone(), TransferPageState::default())
                    }

                    _ => {}
                }

            }

            BankApp::TransferPage(BankState, TransferPageState) => {
                match message {
                    BankMessage::ByPhoneMode => {
                        TransferPageState.transfer_mode = TransferMode::TransferByPhone
                    }

                    BankMessage::ByCardMode => {
                        TransferPageState.transfer_mode = TransferMode::TransferByCard
                    }

                    BankMessage::FromUserChanged(card) => {
                        TransferPageState.sender_card_input = card;
                    }

                    BankMessage::ToUserCardChanged(card) => {
                        TransferPageState.recipient_card_input = card;
                    }

                    BankMessage::ToUserPhoneChanged(phone_num) => {
                        TransferPageState.recipient_phone_input = phone_num;
                    }

                    BankMessage::TransferAmountChanged(amount) => {
                        TransferPageState.amount_input = amount;
                    }

                    BankMessage::TransferByPhone => {
                        let tx = BankState.transfer_by_phone(
                            TransferPageState.sender_card_input.clone(),
                            TransferPageState.recipient_phone_input.clone(),
                            TransferPageState.amount_input.clone()
                        );
                        TransferPageState.reset_inputs()
                    }

                    BankMessage::TransferByCard => {
                        let tx = BankState.transfer_by_card(
                            TransferPageState.sender_card_input.clone(),
                            TransferPageState.recipient_card_input.clone(),
                            TransferPageState.amount_input.clone()
                        );
                        TransferPageState.reset_inputs()
                    }

                    BankMessage::ToUserPage => {
                        *self = BankApp::BankWithUsers(BankState.clone())
                    }

                    _ => {}
                }
            }
        }
    }

    fn view(&self) -> Element<BankMessage> {
        match self {
            BankApp::BeforeBankCreating(InitialBankState {
                input_value }) => {
                let title = text("Bank App")
                    .size(50)
                    .width(Length::Fill)

                    .horizontal_alignment(alignment::Horizontal::Center);

                let input = text_input("Write the name of bank", input_value)
                    .on_input(BankMessage::BankNameChanged)
                    .width(400)
                    .padding(15)
                    .size(30);

                let create_button = button(text("Create"))
                    .padding(20)
                    .style(Button::Primary)
                    .on_press(BankMessage::CreateBank);

                let content = column![title, input, create_button]
                    .spacing(35)
                    .align_items(Alignment::Center)
                    .width(Length::Fill);

                container(content).height(Length::Fill).center_x().align_y(alignment::Vertical::Center).into()


            }

            BankApp::OnlyBankCreated(Bank) => {
                let name = Bank.name.clone();

                let bank_label = text(format!("Your Bank {name} was created"))
                    .size(50);

                let users_label = text("Users are empty...").size(35);

                let user_label = text("Try to create some Users")
                    .size(35);

                let input = |text, input| {
                    let input = text_input(text, input)
                        .width(300)
                        .padding(15)
                        .size(15);
                    input
                };

                let inputs = row![
                    input("Write the FIO", &Bank.input_fio).on_input(BankMessage::UserFioChanged),
                    input("Write the Phone number", &Bank.input_phone).on_input(BankMessage::UserPhoneNumChanged),
                    input("Write the Initial money", &Bank.input_money).on_input(BankMessage::UserMoneyChanged),
                ].spacing(10);


                let create_user_btn = button(text("Create User"))
                    .padding(20)
                    .on_press(BankMessage::FirstUserCreate);

                container(
                    column![
                        Space::with_height(15),
                        bank_label,
                        Space::with_height(30),
                        users_label,
                        Space::with_height(50),
                        user_label,
                        inputs,
                        create_user_btn]
                        .spacing(35)
                        .align_items(Alignment::Center)
                        .width(Length::Fill)
                ).align_x(Horizontal::Center).into()
            }

            BankApp::BankWithUsers(Bank) => {

                let users = Bank.get_users();
                let label = text("Welcome to User's constructor")
                    .size(50)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center);
                let label_user = text("Bank have some Users")
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center);

                let users_vec:Vec<Element<'_,BankMessage, Theme, Renderer>> = {
                    users.iter().map(|(_, user)| user.view().into()).collect()
                };

                let users_container = Row::from_vec(users_vec).spacing(10);

                let scrollable = Scrollable::new(users_container)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .direction(Direction::Horizontal(Properties::new()));

                let input = |text, input| {
                    let input = text_input(text, input)
                        .width(300)
                        .padding(15)
                        .size(15);
                    input
                };

                let inputs = row![
                    input("Write the FIO", &Bank.input_fio).on_input(BankMessage::UserFioChanged),
                    input("Write the Phone number", &Bank.input_phone).on_input(BankMessage::UserPhoneNumChanged),
                    input("Write the Initial money", &Bank.input_money).on_input(BankMessage::UserMoneyChanged),
                ].spacing(10);


                let create_user_btn = button(text("Create User"))
                    .padding(20)
                    .on_press(BankMessage::CreateUser);

                let to_transfer_page_btn = button(text("Переводы ->"))
                    .padding(20)
                    .on_press(BankMessage::ToTransferPage);

                let button_row = row![create_user_btn, to_transfer_page_btn]
                    .spacing(10)
                    .align_items(Alignment::Center);

                let deleted_interface = row![
                    input("Write the Card", &Bank.input_card_num).on_input(BankMessage::UserCardNumChanged),
                    button(text("Delete User"))
                        .padding(20)
                        .on_press(BankMessage::DeleteUser)
                ].align_items(Alignment::Center).spacing(10);

                container(column![Space::with_height(15), label, label_user, scrollable, inputs, button_row, deleted_interface].align_items(Alignment::Center).spacing(35)).align_y(alignment::Vertical::Center).into()


            }

            BankApp::TransferPage(Bank, TransferPageState) => {
                let intro_text = text("Welcome to Transfers")
                    .size(50)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center);

                let phone_mode_btn = button("По номеру телефона")
                    .padding(20)
                    .on_press(BankMessage::ByPhoneMode);

                let card_mode_btn = button("По номеру карты")
                    .padding(20)
                    .on_press(BankMessage::ByCardMode);

                let input = |text, input| {
                    let input = text_input(text, input)
                        .width(300)
                        .padding(15)
                        .size(15);
                    input
                };

                let transfer_by_phone_btn = button("Перевод по номеру")
                    .padding(20)
                    .on_press(BankMessage::TransferByPhone);

                let transfer_by_card_btn = button("Перевод по карте")
                    .padding(20)
                    .on_press(BankMessage::TransferByCard);

                let to_user_page_btn = button("<- Пользователи")
                    .padding(20)
                    .on_press(BankMessage::ToUserPage);

                let to_user_page_btn1 = button("<- Пользователи")
                    .padding(20)
                    .on_press(BankMessage::ToUserPage);

                let phone_mode = column![
                    input("User from...", &TransferPageState.sender_card_input).on_input(BankMessage::FromUserChanged),
                    input("User to (Phone number)...", &TransferPageState.recipient_phone_input).on_input(BankMessage::ToUserPhoneChanged),
                    input("Amount to send...", &TransferPageState.amount_input).on_input(BankMessage::TransferAmountChanged),
                    row![to_user_page_btn1, transfer_by_phone_btn].spacing(5)
                ].spacing(20);

                let card_mode = column![
                    input("User from...", &TransferPageState.sender_card_input).on_input(BankMessage::FromUserChanged),
                    input("User to (Card number)...", &TransferPageState.recipient_card_input).on_input(BankMessage::ToUserCardChanged),
                    input("Amount to send...", &TransferPageState.amount_input).on_input(BankMessage::TransferAmountChanged),
                    row![to_user_page_btn, transfer_by_card_btn].spacing(5)
                ].spacing(20);


                let container = container(
                    column![
                        Space::with_height(20),
                        intro_text,
                        Space::with_height(40),
                        row![card_mode_btn, phone_mode_btn].spacing(5),
                        {
                            match TransferPageState.transfer_mode {
                                TransferMode::TransferByPhone => {
                                    phone_mode
                                },
                                TransferMode::TransferByCard => {
                                    card_mode
                                }
                            }
                        },

                    ].spacing(10).align_items(Alignment::Center)
                ).align_y(Vertical::Center).align_x(Horizontal::Center).into();

                container
            }
        }
    }
}

impl User {
    fn view(&self) -> Container<'_, BankMessage, Theme, Renderer> {
        let card_number = self.check_card_number();
        let fio = self.check_fio();
        let phone_number = self.check_phone();
        let balance = self.check_balance();

        let text_fio = text(format!("Ф.И.О: {fio}")).size(12);
        let text_phone = text(format!("Телефон: {phone_number}")).size(12);
        let text_card_num = text(format!("Номер карты: {card_number}")).size(12);
        let text_balance = text(format!("Баланс: {balance}")).size(12);

        container(column![text_fio, text_phone, text_card_num, text_balance].align_items(Alignment::Start).spacing(10))
            .center_y()
            .width(150)
            .height(120)
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
    }
}

