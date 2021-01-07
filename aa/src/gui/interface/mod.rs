use ag::{audio_graph::AudioGraph};

use ag::{audio_graph, DragEvent};

use iced_baseview::{executor, Align, Application, Command, Subscription, WindowSubs};
use iced_baseview::{
    Column, Element, Row, Container, Rule, 
    Length, Space, Image, image,
    Text, HorizontalAlignment
};
// Import iced_audio modules.
use iced_audio::{
    h_slider, knob, tick_marks, v_slider, xy_pad, FloatRange, FreqRange,
    HSlider, IntRange, Knob, LogDBRange, Normal, VSlider, XYPad,
};

use iced_native::{ button, Button, Color, Point };

#[derive(Debug, Clone)]
pub enum Message {
    Frame,
    VSliderDB(Normal),
    ParameterChange(usize, f64),
    Close(ag::Node),
    Dragged(ag::DragEvent),
    //Clicked(audio_graph::Node),
}
pub struct AAIcedApplication {
    sync_handle: (),

    db_range: LogDBRange,
    v_slider_state: v_slider::State,

    // A group of tick marks with their size and position.
    center_tick_mark: tick_marks::Group,

    nodes: ag::State<Content>,
    nodes_created: usize,
    focus: Option<ag::Node>,
}

impl  Application for AAIcedApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(sync_handle: Self::Flags) -> (Self, Command<Self::Message>) {
        let db_range = LogDBRange::new(-12.0, 12.0, 0.5.into());

        let (mut nodes, _) = 
            ag::State::new(Point::new(0.0, 0.0), Content::new(0));
            nodes.insert(Point::new(400.0,0.0), Content::new(1));
            
        let app = Self {
            db_range,
            
            v_slider_state: v_slider::State::new(
                db_range.default_normal_param(),
            ),
            
            // Add a tick mark at the center position with the tier 2 size
            center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),

            sync_handle,
            nodes,
            nodes_created: 2,
            focus: None,
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
            Message::VSliderDB(normal) => {
                let value = self.db_range.unmap_to_value(normal);
                // self.output_text = format!("VSliderDB: {:.3}", value);
                info!("VSliderDB: {:.3}", value);
            }
            Message::Close(node) => {
                info!("Close {:?}", node);
            }
            Message::Dragged(e) => {
                match e {
                    ag::DragEvent::Dropped{node, diff} => {
                        self.nodes.translate(node, diff);
                    }
                    _ => {}
                }

                info!("Dragged {:?}", e);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> { 
        let title = Text::new("goo")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);
        
        let v_slider_widget =
            VSlider::new(&mut self.v_slider_state, Message::VSliderDB)
                .tick_marks(&self.center_tick_mark);

        let focus = self.focus;
        let total_nodes = self.nodes.len();

        let audio_graph = AudioGraph::new(
            &mut self.nodes, 
            |node, content| {
           
            let is_focused = focus == Some(node);

            let title = Row::with_children(vec![
                Text::new("Node").into(),
                Text::new(content.id.to_string())
                    .color(if is_focused {
                        NODE_ID_COLOR_FOCUSED
                    } else {
                        NODE_ID_COLOR_UNFOCUSED
                    })
                    .into(),
            ])
            .spacing(5)
            .max_width(300);

            let title_bar = ag::TitleBar::new(title)
                .padding(10);
                //.style(style::TitleBar { is_focused });

            ag::Content::new(content.view(node, total_nodes))
                .title_bar(title_bar)
                //.style(style::Pane { is_focused })
        })

        // let audio_graph = AudioGraph::new(
        //     &mut self.nodes,
        //     |node, content| {
        //         ag::Content::new(content.view(node, total_nodes))
        //     })
            .on_drag(Message::Dragged)
            .set_style_sheet(Box::new(audio_graph_style::AudioGraphStyle::new()));

        let all = Column::new()
            .height(Length::Fill)
            .spacing(16)
            .push(title)
            .spacing(20)
            .padding(20)
            .align_items(Align::Center)
            .push(v_slider_widget)
            .push(audio_graph);

        Container::new(all)
            .padding(16)
            .into()
    }
}

const NODE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const NODE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

struct Content {
    id: usize,
    close: button::State,
}

impl Content {
    fn new(id: usize) -> Self {
        Content {
            id,
            close: button::State::new(),
        }
    }
    fn view(
        &mut self,
        node: ag::Node,
        total_panes: usize,
    ) -> Element<Message> {
        let Content {
            close,
            ..
        } = self;

        let button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(style)
        };

        let mut controls = Column::new()
            .max_width(150)
            .push(button(
                close,
                "Close",
                Message::Close(node),
                style::Button::Destructive,
            ));

        Container::new(controls)
            .width(Length::Units(200))
            .height(Length::Units(200))
            .padding(5)
            .center_y()
            .into()
    }
}
mod audio_graph_style {
    use ag::style::style::*;
    use iced_native::{ Color };

    pub struct AudioGraphStyle;
    impl AudioGraphStyle {
        const ACTIVE_STYLE: MusesStyle = MusesStyle {
            fill: Color {
                r: 0.26,
                g: 0.26,
                b: 0.26,
                a: 0.75,
            },
            background: Color::from_rgb(
                0xe0 as f32 / 255.0,
                0xd6 as f32 / 255.0,
                0x44 as f32 / 255.0,
            ),
            border_radius: 0.5,
            border_width: 1.0,
            border_color: Color::from_rgb(
                0xda as f32 / 255.0,
                0x70 as f32 / 255.0,
                0xd6 as f32 / 255.0,
            ),
        };

        pub fn new() -> Self {
            AudioGraphStyle
        }
    }

    impl StyleSheet for AudioGraphStyle {
        fn active(&self) -> Style {
            Style::Muses(Self::ACTIVE_STYLE)
        }

        fn hovered(&self) -> Style {
            Style::Muses(MusesStyle {
                ..Self::ACTIVE_STYLE
            })
        }

        fn dragging(&self) -> Style {
            Style::Muses(MusesStyle {
                ..Self::ACTIVE_STYLE
            })
        }

        fn style(&self) -> Style {
            Style::Muses(MusesStyle {
                ..Self::ACTIVE_STYLE
            })
        }
    }
}

mod style {
    use iced_graphics::{button, container, Background, Color, Vector};

    const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );

    pub enum Button {
        Primary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let (background, text_color) = match self {
                Button::Primary => (Some(ACTIVE), Color::WHITE),
                Button::Destructive => {
                    (None, Color::from_rgb8(0xFF, 0x47, 0x47))
                }
            };

            button::Style {
                text_color,
                background: background.map(Background::Color),
                border_radius: 5.0,
                shadow_offset: Vector::new(0.0, 0.0),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            let background = match self {
                Button::Primary => Some(HOVERED),
                Button::Destructive => Some(Color {
                    a: 0.2,
                    ..active.text_color
                }),
            };

            button::Style {
                background: background.map(Background::Color),
                ..active
            }
        }
    }
}