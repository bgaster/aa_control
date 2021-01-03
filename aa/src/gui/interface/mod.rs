use iced_baseview::{executor, Align, Application, Command, Subscription, WindowSubs};
use iced_baseview::{
    Column, Element, Row, Container, Rule, 
    Length, Space, Image, image,
    Text, HorizontalAlignment
};

#[derive(Debug, Clone)]
pub enum Message {
    Frame,
    ParameterChange(usize, f64),
}
pub struct AAIcedApplication {
    sync_handle: (),
}

impl  Application for AAIcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(sync_handle: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self {
            sync_handle,
        };
        (app, Command::none())
    }

    fn subscription(
        &self,
        window_subs: &mut WindowSubs<Self::Message>) -> Subscription<Self::Message> {
        window_subs.on_frame = Some(Message::Frame);

        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Frame => {
                //self.update_widgets_from_parameters();
            },
            Message::ParameterChange(index, value) => {
                //self.sync_handle.set_parameter(index, value);

                match index {
                    _ => ()
                }

                //self.sync_handle.update_host_display();
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> { 
        let title = Text::new("goo")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);
        
        let all = Column::new()
                    .spacing(16)
                    .push(title);

        Container::new(all)
            .padding(16)
            .into()
    }
}