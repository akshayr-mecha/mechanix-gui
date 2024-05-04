use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::{HorizontalPosition, Styled},
    txt,
    widgets::{Button, Div, IconButton, Svg},
    Color, Node,
};

use crate::{
    components::pin_indicators::PinIndicators,
    gui::{Message, PinKey},
};

pub struct Pin {
    pub pin_length: usize,
}

impl std::fmt::Debug for Pin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Pin")
            .field("pin_length", &self.pin_length)
            .finish()
    }
}

impl Component for Pin {
    fn view(&self) -> Option<Node> {
        let pin_keys = [
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "home",
            "9",
            "0",
            "backspace",
        ];

        let mut pin_buttons = node!(
            Div::new(),
            lay![
                wrap: true,
                size_pct: [100, Auto],
                axis_alignment: Alignment::Center,
            ],
        );

        for (i, pin_key) in pin_keys.into_iter().enumerate() {
            let pin_node: Node = (if pin_key == "home" {
                node!(
                    IconButton::new("home_icon")
                        .on_click(Box::new(|| msg!(Message::PinKeyClicked(PinKey::Home))))
                        .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 20.)
                        .style("padding", 22.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::rgba(42., 42., 44., 0.90)),
                    lay![
                        size: [80, 80],
                        margin: [10],
                    ],
                )
            } else if pin_key == "backspace" {
                node!(
                    IconButton::new("backspace_icon")
                        .on_click(Box::new(|| msg!(Message::PinKeyClicked(PinKey::Backspace))))
                        .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 20.)
                        .style("padding", 22.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::rgba(42., 42., 44., 0.90)),
                    lay![
                        size: [80, 80],
                        margin: [10],
                    ],
                )
            } else {
                node!(
                    Button::new(txt!(pin_key))
                        .on_click(Box::new(|| msg!(Message::PinKeyClicked(PinKey::Text {
                            key: pin_key.to_string()
                        }))))
                        .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 20.)
                        .style("text_color", Color::WHITE)
                        .style("font_size", 32.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::rgba(42., 42., 44., 0.90)),
                    lay![
                        size: [80, 80],
                        margin: [10],
                    ],
                )
            })
            .key(i as u64);
            pin_buttons = pin_buttons.push(pin_node);
        }

        Some(
            node!(
                Div::new(),
                lay![
                    size_pct: [100],
                    direction: Column,
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                PinIndicators {
                    pin_length: self.pin_length,
                },
                lay![
                    margin: [0, 0, 28, 0]
                ]
            ))
            .push(pin_buttons),
        )
    }
}
