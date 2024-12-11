use std::hash::Hash;

use super::component::NetworkRowComponent;
use super::networking::ClicableIconComponent;
use super::wireless_model::WirelessModel;
use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::Image;
use mctk_core::widgets::{Button, Scrollable};
use mctk_core::{
    component::{self, Component},
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{self, Div, IconButton, IconType, Text, Toggle},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};

enum SavedNetworksMessage {
    OpenModel(bool, String),
}

#[derive(Debug, Clone)]
pub struct NetworkSettingsState {
    pub is_model_open: bool,
    pub mac: String,
}

#[derive(Debug)]
#[component(State = "NetworkSettingsState")]
pub struct NetworkSettings {}

impl NetworkSettings {
    pub fn new() -> Self {
        NetworkSettings {
            dirty: false,
            state: Some(NetworkSettingsState {
                is_model_open: false,
                mac: String::from(""),
            }),
        }
    }
}

#[state_component_impl(NetworkSettingsState)]
impl Component for NetworkSettings {
    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.state_ref().is_model_open.hash(hasher);
        self.state_ref().mac.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let is_model_open = self.state_ref().is_model_open;
        let network_name = self.state_ref().mac.clone();

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let text_node = node!(
            Text::new(txt!("Saved Networks"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, Auto],
            ]
        );

        let header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 10],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                margin: [0., 0., 5., 0.],
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::Networking
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    padding: [0, 0, 0, 20.],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(text_node),
        );

        let mut scrollable_section = node!(
            Scrollable::new(),
            lay![
                size: [440, 400],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        ));

        for (i, network) in WirelessModel::get()
            .known_networks
            .get()
            .known_network
            .clone()
            .into_iter()
            .enumerate()
        {
            let mut icon = if network.flags.contains("WPA") {
                "secured_wireless_strong".to_string()
            } else {
                "wireless_strong".to_string()
            };
            let row = node!(
                Div::new(),
                lay![
                    size: [440, 50],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                    // padding: [5., 0., 12., 0.],
                ],
            )
            .push(
                node!(ClicableIconComponent {
                    on_click: Some(Box::new(move || {
                        WirelessModel::select_network(network.network_id.clone());
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::Networking
                            }
                        })
                    }))
                })
                .push(node!(
                    widgets::Image::new(icon),
                    lay![
                        size: [24, 24],
                        margin:[0., 0., 0., 20.],
                    ]
                ))
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, Auto],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                        ]
                    )
                    .push(node!(
                        Text::new(txt!(network.ssid.clone()))
                            .style("color", Color::WHITE)
                            .style("size", 18.0)
                            .style("line_height", 20.0)
                            .style("font", "Space Grotesk")
                            .style("font_weight", FontWeight::Normal),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                            cross_alignment: Alignment::Center,
                        ]
                    )), // .push(node!(
                        //     // mini status
                        //     Text::new(txt!("Saved"))
                        //         .style("color", Color::WHITE)
                        //         .style("size", 14.0)
                        //         .style("line_height", 18.)
                        //         .style("font", "Space Grotesk")
                        //         .style("font_weight", FontWeight::Normal),
                        //     lay![
                        //         direction: Direction::Row,
                        //         axis_alignment: Alignment::Start,
                        //         cross_alignment: Alignment::Center,
                        //     ]
                        // )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [20, Auto],
                        axis_alignment: Alignment::End,
                        cross_alignment:Alignment::Center,
                        padding: [0. , 0., 0., 10.]
                    ]
                )
                .push(node!(
                    IconButton::new("delete_icon")
                        .on_click(Box::new(move || {
                            WirelessModel::forget_saved_network(network.ssid.clone());
                            msg!(())
                        }))
                        // .on_click(Box::new(move || msg!(SavedNetworksMessage::OpenModel(
                        //     !is_model_open,
                        //     network.ssid.clone()
                        // ))))
                        .icon_type(IconType::Png)
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(34.0),
                                height: Dimension::Px(34.0),
                            }
                        )
                        .style("background_color", Color::TRANSPARENT)
                        .style("border_color", Color::TRANSPARENT)
                        .style("active_color", Color::rgba(85., 85., 85., 0.50))
                        .style("radius", 10.),
                    lay![
                        size: [52, 52],
                        axis_alignment: Alignment::End,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            )
            .key(i as u64);

            let row = node!(
                Div::new(),
                lay![
                    size: [440, 50],
                    direction: Direction::Column,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                    // padding: [5., 0., 12., 0.],
                ],
            )
            .push(row)
            .push(node!(HDivider { size: 1.0 }))
            .key(2 * i as u64);

            scrollable_section = scrollable_section.push(row);
        }

        let modal = node!(
            Div::new()
                .bg(Color::DARK_GREY)
                .border(Color::DARK_GREY, 1., (10., 10., 10., 10.)),
            lay![
                size: [280, 180],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                position_type: Absolute,
                position: [120., 80., 0., 0.],
            ]
        )
        .push(
            node!(
                Div::new().border(Color::TRANSPARENT, 1., (10., 10., 10., 10.)),
                // Div::new(),
                lay![
                size_pct: [100, 70],
                direction: Direction::Row,
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center,
                padding:[0., 20., 0., 0.]
                ]
            )
            .push(node!(
                Text::new(txt!("Forget this network? "))
                    .style("color", Color::WHITE)
                    .style("size", 20.)
                    .style("line_height", 22.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    size_pct: [100, 50],
                ],
            )),
        )
        .push(
            node!(
                Div::new().border(Color::TRANSPARENT, 1.5, (0., 10., 10., 10.)),
                lay![
                    size_pct: [100, 30],
                    direction: Direction::Row,
                    cross_alignment: Alignment::Stretch,
                    axis_alignment: Alignment::Stretch,
                    padding: [0., 0., 5., 0.]
                ]
            )
            .push(node!(
                Button::new(txt!("Cancel"))
                    .style("text_color", Color::WHITE)
                    .style("background_color", Color::DARK_GREY)
                    .style("active_color", Color::MID_GREY)
                    .style("font_size", 20.)
                    .style("line_height", 22.)
                    .on_click(Box::new(move || msg!(SavedNetworksMessage::OpenModel(
                        !is_model_open,
                        "".to_string()
                    )))),
                lay![
                    size_pct: [48, Auto],
                ]
            ))
            .push(
                node!(
                    Div::new().bg(Color::TRANSPARENT),
                    lay![
                     size_pct: [4, Auto],
                     axis_alignment: Alignment::Center,
                     cross_alignment: Alignment::Center
                    ]
                )
                .push(node!(
                    Text::new(txt!("|"))
                        .style("color", Color::LIGHT_GREY)
                        .style("size", 20.)
                        .style("line_height", 22.)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        cross_alignment: Alignment::Center
                    ]
                )),
            )
            .push(node!(
                Button::new(txt!("Forget"))
                    .style("text_color", Color::RED)
                    .style("background_color", Color::DARK_GREY)
                    .style("active_color", Color::MID_GREY)
                    .style("font_size", 20.)
                    .style("line_height", 22.)
                    // .on_click(Box::new(move || {
                    //     WirelessModel::forget_saved_network(network.ssid.clone());
                    //     msg!(())
                    // })),
                    .on_click(Box::new(move || {
                        WirelessModel::forget_saved_network(network_name.clone());
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::NetworkSettings
                            }
                        })
                    })),
                lay![
                    size_pct: [48, Auto],
                ]
            )),
        );

        base = base.push(header_node);
        base = base.push(scrollable_section);

        // if is_model_open.clone() == true {
        // base = base.push(modal);
        // base = base.push(node!(Div::new().bg(Color::RED), lay![size: [440, 400]]));

        // }
        Some(base)
    }

    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<mctk_core::component::Message> {
        if let Some(message) = msg.downcast_ref::<SavedNetworksMessage>() {
            match message {
                SavedNetworksMessage::OpenModel(value, mac) => {
                    self.state_mut().is_model_open = *value;
                    self.state_mut().mac = mac.clone();
                }
            }
        }
        vec![msg]
    }
}
