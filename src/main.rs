// gui libraries
use iced::{Application, Command};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, container, Container},
    Length, Settings,
};

// libs for chat funtionality
use futures::stream::StreamExt;
use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{io, io::AsyncBufReadExt, select};
use tracing_subscriber::EnvFilter;

// structure for the gui
struct LanChat;

// structure for custom network behaviour that combines Gossipsub and Mdns.
struct MyBehavior{
    gossipsub: gossipsub::Behavior,
    mdns: mdns::tokio::Behavior,
}

impl Application for LanChat {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = ();

    fn new(_flags: ()) -> (LanChat, Command<Self::Message>) {
        (LanChat, Command::none())
    }

    fn title(&self) -> String {
        String::from(" andi's LAN Chat")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        iced::Text::new("Hello, LAN Chat!").into()
    }
}

fn render_ui() -> iced::Result{
    LanChat::run(iced::Settings::default())
}

#[tokio::main]
async fn main() {
    render_ui()
}
