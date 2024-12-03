use std::hash::Hash;

use crate::gui::Message;
use crate::shared::close_button::CloseButton;
use crate::shared::h_divider::HDivider;
use crate::types::{RestartState, ShutdownState};
use crate::{AppMessage, AppParams};

use mctk_core::layout::{Alignment, Dimension};

use mctk_core::renderables::{Image, Renderable};
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Button, IconButton, Text};
use mctk_core::{component, layout, Color, Scale, AABB};
use mctk_core::{
    component::Component, lay, msg, node, rect, size, size_pct, state_component_impl, txt,
    widgets::Div, Node,
};

#[derive(Debug)]
pub struct PowerOptions {
    pub shutdown_pressed: bool,
    pub restart_pressed: bool,
}

impl Component for PowerOptions {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.shutdown_pressed.hash(hasher);
        self.restart_pressed.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().bg(Color::BLACK),
                lay![
                    size_pct: [100],
                    cross_alignment: Alignment::Stretch,
                    direction: layout::Direction::Column,
                    padding: [20., 20., 0., 20.]
                ]
            )
            .push(node!(HDivider { size: 1. }))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size: [Auto, 378],
                        cross_alignment: Alignment::Stretch,
                        axis_alignment: Alignment::Center,
                        direction: layout::Direction::Column,
                        padding: [0., 20., 0., 0.]
                    ]
                )
                .push(
                    node!(
                        Div::new(),
                        lay![
                            direction: layout::Direction::Row,
                            cross_alignment: Alignment::Center
                        ]
                    )
                    .push(node!(
                        IconButton::new("shutdown_icon")
                            .icon_type(mctk_core::widgets::IconType::Png)
                            .on_click(Box::new(|| msg!(Message::Shutdown(ShutdownState::Clicked))))
                            .on_press(Box::new(|| msg!(Message::Shutdown(ShutdownState::Pressed))))
                            .on_release(Box::new(|| msg!(Message::Shutdown(
                                ShutdownState::Released
                            ))))
                            .with_class("p-4 border-2")
                            .style(
                                "background_color",
                                if self.shutdown_pressed {
                                    Color::rgba(243., 24., 65., 0.30)
                                } else {
                                    Color::TRANSPARENT
                                }
                            )
                            .style(
                                "active_color",
                                if self.shutdown_pressed {
                                    Color::rgba(243., 24., 65., 0.30)
                                } else {
                                    Color::TRANSPARENT
                                }
                            )
                            .style(
                                "border_color",
                                if self.shutdown_pressed {
                                    Color::TRANSPARENT
                                } else {
                                    Color::rgb(243., 24., 65.)
                                }
                            )
                            .style("size", size!(80, 80)),
                        lay![
                            margin: [0., 0., 0., 14.],
                        ]
                    ))
                    .push(node!(
                        Button::new(txt!("Power Off"))
                            .on_click(Box::new(|| msg!(Message::Shutdown(ShutdownState::Clicked))))
                            .on_press(Box::new(|| msg!(Message::Shutdown(ShutdownState::Pressed))))
                            .on_release(Box::new(|| msg!(Message::Shutdown(
                                ShutdownState::Released
                            ))))
                            .with_class(
                                "text-3xl leading-7 bg-transparent font-medium font-space-grotesk p-0"
                            )
                            .style("text_color", Color::rgb(243., 24., 65.))
                            .style("active_color", Color::TRANSPARENT)
                            ,
                        lay![
                            size: [131, 36]
                        ]
                    )),
                )
                .push(
                    node!(
                        Div::new(),
                        lay![
                            direction: layout::Direction::Row,
                            cross_alignment: Alignment::Center,
                            margin: [30., 0., 0., 0.]
                        ]
                    )
                    .push(node!(
                        IconButton::new("restart_icon")
                            .icon_type(mctk_core::widgets::IconType::Png)
                            .on_click(Box::new(|| msg!(Message::Restart(RestartState::Clicked))))
                            .on_press(Box::new(|| msg!(Message::Restart(RestartState::Pressed))))
                            .on_release(Box::new(|| msg!(Message::Restart(RestartState::Released))))
                            .with_class("p-4 border-2")
                            .style(
                                "background_color",
                                if self.restart_pressed {
                                    Color::rgb(43., 43., 43.)
                                } else {
                                    Color::TRANSPARENT
                                }
                            )
                            .style(
                                "active_color",
                                if self.restart_pressed {
                                    Color::rgb(43., 43., 43.)
                                } else {
                                    Color::TRANSPARENT
                                }
                            )
                            .style(
                                "border_color",
                                if self.restart_pressed {
                                    Color::TRANSPARENT
                                } else {
                                    Color::WHITE
                                }
                            )
                            .style("size", size!(80, 80)),
                        lay![
                            margin: [0., 0., 0., 0.],
                        ]
                    ))
                    .push(node!(
                        Button::new(txt!("Restart"))
                            .with_class(
                                "text-3xl leading-7 text-white bg-transparent font-medium font-space-grotesk p-0"
                            )
                            .on_click(Box::new(|| msg!(Message::Restart(RestartState::Clicked))))
                            .on_press(Box::new(|| msg!(Message::Restart(RestartState::Pressed))))
                            .on_release(Box::new(|| msg!(Message::Restart(RestartState::Released))))
                            .style("active_color", Color::TRANSPARENT),
                        lay![
                            size: [131, 36]
                        ]
                    )),
                ),
            )
            .push(node!(HDivider { size: 1. }))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size: [Auto, 78],
                        axis_alignment: Alignment::End,
                        cross_alignment: Alignment::Center,
                    ]
                )
                .push(node!(
                    CloseButton::new()
                        .on_click(Box::new(|| msg!(Message::PowerOptions { show: false }))),
                    lay![
                        size: [114, 50],
                    ]
                )),
            ),
        )
    }
}
