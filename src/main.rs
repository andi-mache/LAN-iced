
use iced::widget::{button, column,row, container, text,text_input, Column, Button, vertical_space, horizontal_space, vertical_rule, horizontal_rule};
use iced::{Center, Element, Task, Theme, Fill};
use iced::widget::scrollable;
use iced::widget::pick_list;
pub fn main() -> iced::Result {
    iced::application("LAN-chat", App::update, App::view)
        
        .theme(App::theme)
        .run_with(App::new)

}

//#[derive(Default)]
struct App {
    theme: Theme,
    screen: Screen,
    value: i64,
    user: String,
    pass: String,
    nick: String,
    msg: String,
    chat_history: String,

}

#[derive(Debug, Clone)]
enum Message {
    LoginButtonPressed,
    ThemeSelected(Theme),
    BackPressed,
    Increment,
    Decrement,
    SendMsg,
    UserInput(String),
    PassInput(String),
    ChatInput(String),
   
}

impl App {
    fn new() -> (Self, Task<Message>) {
        ( Self {
            theme: Theme::Dark,
            screen: Screen::Login,
            value: 0,
            user: String::new(),
            pass: String::new(),
            nick: String::new(),
            msg: String::new(),
            chat_history: String::new(),
    
        },
        Task::none(),
        )
    }


    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;

                Task::none()
            }
            Message::ChatInput(s) => {
                self.msg = s;

                Task::none()
            }
            Message::SendMsg => {
                // Implement actual message sending logic here (e.g., network communication)
                self.chat_history.push_str(&format!("[{}] says {}\n", self.nick, self.msg));
                self.msg.clear();

                Task::none()
            }
            Message::UserInput(s) => {
                self.user = s;

                Task::none()

            }
            Message::PassInput(s) => {
                self.pass = s;

                Task::none()
            }
            Message::BackPressed => {
                if let Some(screen) = self.screen.previous() {
                    self.screen = screen;
                }


                Task::none()
            }
            Message::LoginButtonPressed => {
                if let Some(screen) = self.screen.next() {
                    self.screen = screen;
                }

                Task::none() 
            }
            Message::Increment => {
                self.value += 1;

                Task::none()

            }
            Message::Decrement => {
            
                self.value -= 1;

                Task::none()
            }

        }
    }
    fn view(&self) -> Element<Message> {
        let screen = match self.screen {
            Screen::Login => self.login(),
            Screen::Home => self.home(),
            Screen::End => self.end(),
            
        };

        let content: Element<_> = column![screen,]
        .width(Fill)
        .spacing(20)
        .padding(20)
        .into();

        container(content).center(Fill).into()

    }

    //async fn connect_to_server(&mut self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    //    let socket = TcpStream::connect(addr).await?;
    //    let (reader, writer) = socket.into_split();
    //
    //    tokio::spawn(async move {
    //        let mut reader = BufReader::new(reader);
    //        loop {
    //            let mut buffer = String::new();
    //            reader.read_line(&mut buffer).await.unwrap();
    //            self.chat_history.push_str(&buffer);
    //        }
    //    });
    //
    //    self.socket = Some(writer);
    //    Ok(())
    //}
    //
    //fn can_continue(&self) -> bool {
    //    match self.screen {
    //        Screen::Login => true,
    //        Screen::Home => true,
    //        Screen::End => false,
    //    }
    //}

    fn end(&self) -> Column<Message> {
        Self::container("You reached the end!")
            .push("This tour will be updated as more features are added.")
            .push("Make sure to keep an eye on it!")
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
         }
    fn login(&self) -> Column<Message> {

        let user_input = text_input("username", &self.user).width(400).on_input(Message::UserInput);
        let pass_input = text_input("password", &self.pass).width(400);
        let submit_button = button("submit").on_press(Message::LoginButtonPressed);
        let register_button = button("register_button");
        let input_controls = row![register_button,submit_button].padding(20).spacing(20);

        column![
            text("yoy").size(70),
            user_input,
            pass_input,
            input_controls,
            vertical_space(),
            //padded_button("Next").on_press(Message::LoginButtonPressed),
        ].width(Fill).align_x(Center).padding(20).spacing(20)        
        
    }

    fn home(&self) -> Column<Message> {

        let topcontrols = row![
            button("one"),button("two"),
            button("BackPressed").on_press(Message::BackPressed),
            pick_list(
                Theme::ALL,
                Some(self.theme.clone()),
                Message::ThemeSelected
            )
            .text_size(14)
            .padding([5, 10])
        ].spacing(20);
        let sidebar = column![button("sdsds"), button("sees")].spacing(20);
        let chat_controls = row![
            text_input("Nick",&self.nick).width(75),
            text_input("message",&self.msg),
            button("Send").on_press(Message::SendMsg),    
        ].spacing(5);
        let chat_history = scrollable(text(&self.chat_history));
        //let body
        let content = column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
            chat_history,
            vertical_space(),
            horizontal_rule(10),
            chat_controls,
        ].align_x(Center);
        let main_body = row![sidebar, vertical_rule(10), content, vertical_rule(10)];
        column![
            topcontrols,
            horizontal_rule(10),
            main_body,
            horizontal_rule(10),
                   ]
       
    }

        fn container(title: &str) -> Column<'_, Message> {
        column![text(title).size(50)].spacing(20)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Login,
    Home,
    End,
}

impl Screen {
    const ALL: &'static [Self] = &[
        Self::Login,
        Self::Home,
        Self::End,
    ];

    pub fn next(self) -> Option<Screen> {
        Self::ALL
            .get(
                Self::ALL
                    .iter()
                    .copied()
                    .position(|screen| screen == self)
                    .expect("Screen must exist")
                    + 1,
            )
            .copied()
    }

    pub fn previous(self) -> Option<Screen> {
        let position = Self::ALL
            .iter()
            .copied()
            .position(|screen| screen == self)
            .expect("Screen must exist");

        if position > 0 {
            Some(Self::ALL[position - 1])
        } else {
            None
        }
    }
}


