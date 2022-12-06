use dotenv::dotenv;

// This bot throws a dice on each incoming message.

use std::time::Duration;
use teloxide::prelude::*;
use tokio::time::sleep;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let db_client = match connect_to_database().await {
        Err(error) => panic!("Problem connecting to the database: {:?}", error),
        Ok(db_client) => db_client,
    };

    // let telegram_id = "telegram_id";
    // let lastname = "lastname";
    // let profcom_number = "profcom_number";
    // let number = match db_client.execute(
    //     "INSERT INTO printer_tgbot_test (telegram_id, lastname, profcom_number) VALUES ($1, $2, $3)",
    //     &[&telegram_id, &lastname, &profcom_number],
    // ).await {
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    //     Ok(number) => number,
    // };
    // println!("execute {number}");

    let chat = "123872139";
    let rows = match db_client
        .query("SELECT * FROM printer_tgbot_test WHERE telegram_id = $1", &[&chat])
        .await
    {
        Err(error) => panic!("Problem query: {:?}", error),
        Ok(rows) => rows,
    };

    if rows.len() == 0 {
        let telegram_id = chat;
        let lastname = "lastname";
        let profcom_number = "profcom_number";
        let _number = match db_client.execute(
            "INSERT INTO printer_tgbot_test (telegram_id, lastname, profcom_number) VALUES ($1, $2, $3)",
            &[&telegram_id, &lastname, &profcom_number],
        ).await {
            Err(error) => panic!("Problem opening the file: {:?}", error),
            Ok(number) => number,
        };
    };
    println!("{chat}");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // let chat_id = msg.from().unwrap().id;
        // let chat = format!("{chat_id}");

        // for row in rows{
        //     let id: i32 = row.get(0);
        //     let name: &str = row.get(1);
        //     let data: Option<&[u8]> = row.get(2);

        //     println!("found person: {} {} {:?}", id, name, data);
        // };

        if let Some(text) = msg.text() {
            bot.send_message(msg.chat.id, text).await?;
        }
        sleep(Duration::from_millis(20000)).await;
        // match msg.text() {
        //     Some(text) => {
        //         bot.send_message(msg.chat.id, text).await?;
        //     }
        //     None => ()
        // }
        Ok(())
    })
    .await;
}

async fn connect_to_database() -> Result<tokio_postgres::Client, Error> {
    let host = dotenv::var("DB_HOST").unwrap();
    let port = dotenv::var("DB_PORT").unwrap();
    let user = dotenv::var("DB_USER").unwrap();
    let password = dotenv::var("DB_PASSWORD").unwrap();
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        format!("host={host} port={port} user={user} password={password} dbname=dev").as_str(),
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

async fn check_id() -> bool {
    true
}

// // This is a bot that asks you three questions, e.g. a simple test.
// //
// // # Example
// // ```
// //  - Hey
// //  - Let's start! What's your full name?
// //  - Gandalf the Grey
// //  - How old are you?
// //  - 223
// //  - What's your location?
// //  - Middle-earth
// //  - Full name: Gandalf the Grey
// //    Age: 223
// //    Location: Middle-earth
// // ```
// use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

// type MyDialogue = Dialogue<State, InMemStorage<State>>;
// type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

// #[derive(Clone, Default)]
// pub enum State {
//     #[default]
//     Start,
//     ReceiveFullName,
//     ReceiveAge {
//         full_name: String,
//     },
//     ReceiveLocation {
//         full_name: String,
//         age: u8,
//     },
// }

// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//     pretty_env_logger::init();
//     log::info!("Starting dialogue bot...");

//     let bot = Bot::from_env();

//     Dispatcher::builder(
//         bot,
//         Update::filter_message()
//             .enter_dialogue::<Message, InMemStorage<State>, State>()
//             .branch(dptree::case![State::Start].endpoint(start))
//             .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
//             .branch(dptree::case![State::ReceiveAge { full_name }].endpoint(receive_age))
//             .branch(
//                 dptree::case![State::ReceiveLocation { full_name, age }].endpoint(receive_location),
//             ),
//     )
//     .dependencies(dptree::deps![InMemStorage::<State>::new()])
//     .enable_ctrlc_handler()
//     .build()
//     .dispatch()
//     .await;
// }

// async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//     bot.send_message(msg.chat.id, "Let's start! What's your full name?").await?;
//     dialogue.update(State::ReceiveFullName).await?;
//     Ok(())
// }

// async fn receive_full_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//     match msg.text() {
//         Some(text) => {
//             bot.send_message(msg.chat.id, "How old are you?").await?;
//             dialogue.update(State::ReceiveAge { full_name: text.into() }).await?;
//         }
//         None => {
//             bot.send_message(msg.chat.id, "Send me plain text.").await?;
//         }
//     }

//     Ok(())
// }

// async fn receive_age(
//     bot: Bot,
//     dialogue: MyDialogue,
//     full_name: String, // Available from `State::ReceiveAge`.
//     msg: Message,
// ) -> HandlerResult {
//     match msg.text().map(|text| text.parse::<u8>()) {
//         Some(Ok(age)) => {
//             bot.send_message(msg.chat.id, "What's your location?").await?;
//             dialogue.update(State::ReceiveLocation { full_name, age }).await?;
//         }
//         _ => {
//             bot.send_message(msg.chat.id, "Send me a number.").await?;
//         }
//     }

//     Ok(())
// }

// async fn receive_location(
//     bot: Bot,
//     dialogue: MyDialogue,
//     (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
//     msg: Message,
// ) -> HandlerResult {
//     match msg.text() {
//         Some(location) => {
//             let report = format!("Full name: {full_name}\nAge: {age}\nLocation: {location}");
//             bot.send_message(msg.chat.id, report).await?;
//             dialogue.exit().await?;
//         }
//         None => {
//             bot.send_message(msg.chat.id, "Send me plain text.").await?;
//         }
//     }

//     Ok(())
// }

// // Derive BotCommands to parse text with a command into this enumeration.
// //
// // 1. `rename_rule = "lowercase"` turns all the commands into lowercase letters.
// // 2. `description = "..."` specifies a text before all the commands.
// //
// // That is, you can just call Command::descriptions() to get a description of
// // your commands in this format:
// // %GENERAL-DESCRIPTION%
// // %PREFIX%%COMMAND% - %DESCRIPTION%
// #[derive(BotCommands, Clone)]
// #[command(
//     rename_rule = "lowercase",
//     description = "Use commands in format /%command% %num% %unit%",
//     parse_with = "split"
// )]

// use std::error::Error;
// use teloxide::{
//     payloads::SendMessageSetters,
//     prelude::*,
//     types::{
//         InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
//         InputMessageContentText, Me,
//     },
//     utils::command::BotCommands,
// };

// #[derive(BotCommands)]
// #[command(
//     rename_rule = "lowercase",
//     description = "These commands are supported:"
// )]
// enum Command {
//     #[command(description = "Display this text")]
//     Help,
//     #[command(description = "Start")]
//     Start,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     dotenv().ok();
//     pretty_env_logger::init();
//     log::info!("Starting buttons bot...");

//     let bot = Bot::from_env();

//     let handler = dptree::entry()
//         .branch(Update::filter_message().endpoint(message_handler))
//         .branch(Update::filter_callback_query().endpoint(callback_handler))
//         .branch(Update::filter_inline_query().endpoint(inline_query_handler));

//     Dispatcher::builder(bot, handler)
//         .enable_ctrlc_handler()
//         .build()
//         .dispatch()
//         .await;
//     Ok(())
// }

// /// Creates a keyboard made by buttons in a big column.
// fn make_keyboard() -> InlineKeyboardMarkup {
//     let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

//     let debian_versions = [
//         "Buzz", "Rex", "Bo", "Hamm", "Slink", "Potato", "Woody", "Sarge", "Etch", "Lenny",
//         "Squeeze", "Wheezy", "Jessie", "Stretch", "Buster", "Bullseye",
//     ];

//     for versions in debian_versions.chunks(3) {
//         let row = versions
//             .iter()
//             .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
//             .collect();

//         keyboard.push(row);
//     }

//     InlineKeyboardMarkup::new(keyboard)
// }

// /// Parse the text wrote on Telegram and check if that text is a valid command
// /// or not, then match the command. If the command is `/start` it writes a
// /// markup with the `InlineKeyboardMarkup`.
// async fn message_handler(
//     bot: Bot,
//     msg: Message,
//     me: Me,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     if let Some(text) = msg.text() {
//         match BotCommands::parse(text, me.username()) {
//             Ok(Command::Help) => {
//                 // Just send the description of all commands.
//                 bot.send_message(msg.chat.id, Command::descriptions().to_string())
//                     .await?;
//             }
//             Ok(Command::Start) => {
//                 // Create a list of buttons and send them.
//                 let keyboard = make_keyboard();
//                 bot.send_message(msg.chat.id, "Debian versions:")
//                     .reply_markup(keyboard)
//                     .await?;
//             }

//             Err(_) => {
//                 bot.send_message(msg.chat.id, "Command not found!").await?;
//             }
//         }
//     }

//     Ok(())
// }

// async fn inline_query_handler(
//     bot: Bot,
//     q: InlineQuery,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let choose_debian_version = InlineQueryResultArticle::new(
//         "0",
//         "Chose debian version",
//         InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
//     )
//     .reply_markup(make_keyboard());

//     bot.answer_inline_query(q.id, vec![choose_debian_version.into()])
//         .await?;

//     Ok(())
// }

// /// When it receives a callback from a button it edits the message with all
// /// those buttons writing a text with the selected Debian version.
// ///
// /// **IMPORTANT**: do not send privacy-sensitive data this way!!!
// /// Anyone can read data stored in the callback button.
// async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
//     if let Some(version) = q.data {
//         let text = format!("You chose: {version}");

//         // Tell telegram that we've seen this query, to remove 🕑 icons from the
//         //
//         // clients. You could also use `answer_callback_query`'s optional
//         // parameters to tweak what happens on the client side.
//         bot.answer_callback_query(q.id).await?;

//         // Edit text of the message to which the buttons were attached
//         if let Some(Message { id, chat, .. }) = q.message {
//             bot.edit_message_text(chat.id, id, text).await?;
//         } else if let Some(id) = q.inline_message_id {
//             bot.edit_message_text_inline(id, text).await?;
//         }

//         log::info!("You chose: {}", version);
//     }

//     Ok(())
// }

// use teloxide::{prelude::*, utils::command::BotCommands};

// #[tokio::main]
// async fn main() {
//     pretty_env_logger::init();
//     log::info!("Starting command bot...");

//     let bot = Bot::from_env();

//     Command::repl(bot, answer).await;
// }

// #[derive(BotCommands, Clone)]
// #[command(
//     rename_rule = "lowercase",
//     description = "These commands are supported:"
// )]
// enum Command {
//     #[command(description = "display this text.")]
//     Help,
//     #[command(description = "handle a username.")]
//     Username(String),
//     #[command(description = "handle a username and an age.", parse_with = "split")]
//     UsernameAndAge { username: String, age: u8 },
// }

// async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
//     match cmd {
//         Command::Help => {
//             bot.send_message(msg.chat.id, Command::descriptions().to_string())
//                 .await?
//         }
//         Command::Username(username) => {
//             bot.send_message(msg.chat.id, format!("Your username is @{username}."))
//                 .await?
//         }
//         Command::UsernameAndAge { username, age } => {
//             bot.send_message(
//                 msg.chat.id,
//                 format!("Your username is @{username} and age is {age}."),
//             )
//             .await?
//         }
//     };

//     Ok(())
// }
