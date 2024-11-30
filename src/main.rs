
use iced::widget::{button, column,row, container, text,text_input, Column, Button, vertical_space, horizontal_space, vertical_rule, horizontal_rule};
use iced::{Center, Element, Task, Theme, Fill};

pub fn main() -> iced::Result {
    iced::application("LAN-chat", App::update, App::view)
        
        .theme(App::theme)
        .run_with(App::new)

}

//#[derive(Default)]
struct App {
    screen: Screen,
    value: i64,

}

#[derive(Debug, Clone, Copy)]
enum Message {
    LoginButtonPressed,
    BackPressed,
    Increment,
    Decrement,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        ( Self {
            screen: Screen::Login,
            value: 0,
            
    
        },
        Task::none(),
        )
    }


    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
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
    fn can_continue(&self) -> bool {
        match self.screen {
            Screen::Login => true,
            Screen::Home => true,
            Screen::End => false,
        }
    }

    fn end(&self) -> Column<Message> {
        Self::container("You reached the end!")
            .push("This tour will be updated as more features are added.")
            .push("Make sure to keep an eye on it!")
    }

    fn theme(&self) -> Theme {
                   Theme::Dark
         }
    fn login(&self) -> Column<Message> {

        let user_input = text_input("username", "").width(400);
        let pass_input = text_input("password", "").width(400);
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
            button("one"),button("two")       ,     button("BackPressed").on_press(Message::BackPressed),
        ].spacing(20);
        let sidebar = column![button("sdsds"), button("sees")].spacing(20);
        let chat_controls = row![
            text_input("Nick","").width(75),
            text_input("message",""),
            button("Send"),
    
        ].spacing(5);
        //let body
        let content = column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
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

fn padded_button<Message: Clone>(label: &str) -> Button<'_, Message> {
    button(text(label)).padding([12, 24])
}
