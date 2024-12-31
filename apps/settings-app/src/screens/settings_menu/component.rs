use std::fmt::Debug;

use mctk_core::{
    component::{Component, Message},
    event, lay,
    layout::{Alignment, Direction},
    node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, IconType, Image, Svg, Text},
    Color,
};

use crate::components::get_icon;

pub struct SettingsRowComponent {
    pub title: String,
    pub value: String,
    pub icon_1: String,
    pub icon_1_type: IconType,
    pub icon_2: String,
    pub color: Color,
    pub on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl Debug for SettingsRowComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsRowComponent")
            .field("title", &self.title)
            .field("icon_1", &self.icon_1)
            .field("icon_1_type", &self.icon_1_type)
            .field("icon_2", &self.icon_2)
            .finish()
    }
}

impl Component for SettingsRowComponent {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn view(&self) -> Option<node::Node> {
        let title = self.title.clone();
        let value = self.value.clone();
        let icon_1 = self.icon_1.clone();
        let icon_1_type = self.icon_1_type.clone();
        let icon_2 = self.icon_2.clone();
        let color = self.color.clone();

        let text_node = node!(Text::new(txt!(title))
            .style("color", color)
            .style("font", "Inter")
            .with_class("text-2xl leading-7 font-bold"));

        let value_node = node!(
            Text::new(txt!(value))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("font", "Inter")
                .with_class("text-xl leading-6 font-normal"),
            lay![
                margin: [0., 0., 0., 10.],
            ]
        );

        Some(
            node!(
                Div::new(),
                lay![
                    size: [440, 68],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [70, Auto],
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ],
                )
                .push(get_icon(&icon_1, icon_1_type, rect![0., 10., 0., 20.]))
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, Auto],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                        ]
                    )
                    .push(text_node),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [30, Auto],
                        axis_alignment: Alignment::End,
                        cross_alignment:Alignment::Center,
                    ]
                )
                .push(value_node),
                // .push(get_icon(&icon_2, IconType::Svg, rect![0., 0., 0., 10.])),
            ),
        )
    }
}
