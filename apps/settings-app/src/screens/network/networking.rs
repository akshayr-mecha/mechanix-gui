use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::Image;
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

use mechanix_system_dbus_client::wireless::WirelessInfoResponse;
use zbus::message;

enum NetworkingMessage {
    handleClickOnMore,
    handleClickOnBack,
}

#[derive(Debug)]
pub struct NetworkScreenState {
    // pub loading: bool,
    pub status: bool,
    pub name: String,
    pub more_clicked: bool,
    // pub app_channel: Option<Sender<AppMessage>>,
    // pub connected_network: Option<WirelessDetailsItem>,
}

#[derive(Debug)]
#[component(State = "NetworkScreenState")]
pub struct NetworkingScreen {}

impl NetworkingScreen {
    // pub fn new(status: bool, connected_network: Option<WirelessDetailsItem>) -> Self {
    pub fn new(status: bool, name: String) -> Self {
        Self {
            state: Some(NetworkScreenState {
                status,
                name,
                more_clicked: false,
                // connected_network: connected_network,
            }),
            dirty: false,
        }
    }
}

impl Component for NetworkingScreen {
    fn view(&self) -> Option<Node> {
        let mut text_color = Color::WHITE;

        let connected_network_name: String = self.state_ref().name.clone();

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
            Text::new(txt!("Networking"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, Auto],
                margin:[2.0, 5.0, 2.0, 5.0],
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
                // position_type: Absolute,
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [60, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::SettingsList
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
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [40, Auto],
                    axis_alignment: Alignment::End,
                    padding: [0, 0, 0, 0.],
                ]
            )
            .push(node!(
                IconButton::new("add_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::AddNetwork
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(40.0),
                            height: Dimension::Px(40.0),
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
            ))
            .push(node!(
                IconButton::new("network_settings_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkSettings
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
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 90],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        // toggle row
        let toggle_row = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment:Alignment::Center,
                padding: [5., 0., 15., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Text::new(txt!("Wireless"))
                    .style("color", Color::WHITE)
                    .style("size", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(Toggle::new(true), lay![])),
        );

        let toggle_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(toggle_row);

        let connected_network_row = node!(
            Div::new(),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                widgets::Image::new("wifi_icon"),
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
                    Text::new(txt!("Mecha Workstation"))
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
                ))
                .push(node!(
                    // mini status
                    Text::new(txt!("Connected"))
                        .style("color", Color::WHITE)
                        .style("size", 14.0)
                        .style("line_height", 18.)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                )),
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
                IconButton::new("info_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkDetails
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
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        // avialble networks
        let available_network_text = node!(
            Text::new(txt!("Available Networks"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 16.0)
                .style("line_height", 18.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [30.0, 0.0, 10.0, 0.0],
            ]
        );

        let available_network_row_1 = node!(
            Div::new(),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                widgets::Image::new("wifi_icon"),
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
                    Text::new(txt!("Mecha Admin"))
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
                ))
                .push(node!(
                    // mini status
                    Text::new(txt!("Saved"))
                        .style("color", Color::WHITE)
                        .style("size", 14.0)
                        .style("line_height", 18.)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                )),
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
                // TODO: pass selected network details
                IconButton::new("info_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkDetails
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
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        let available_network_row_2 = node!(
            Div::new(),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                widgets::Image::new("wifi_icon"),
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
                    Text::new(txt!("Mecha Guest"))
                        .style("color", Color::WHITE)
                        .style("size", 18.0)
                        .style("line_height", 20.0)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                    ]
                )),
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
                IconButton::new("info_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkDetails
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
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        let available_network_row_3 = node!(
            Div::new(),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                widgets::Image::new("wifi_icon"),
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
                    Text::new(txt!("Mecha Guest"))
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
                )),
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
                IconButton::new("info_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkDetails
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
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        let view_all_text: Node = node!(
            Text::new(txt!("view all"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 10.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [2.0, 0.0, 2.0, 0.0],
                axis_alignment: Alignment::Start
            ]
        );

        // // Advanced
        let advanced_nextwork_text = node!(
            Text::new(txt!("Advanced"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 16.0)
                .style("line_height", 18.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [20.0, 0.0, 10.0, 0.0],
            ]
        );

        let advanced_network_row = node!(
            Div::new(),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Text::new(txt!("Add Network"))
                    .style("color", Color::rgba(45., 138., 225., 1.))
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    direction: Direction::Row,
                    axis_alignment: Alignment::Start,
                ]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End,
                    padding: [0. , 0., 0., 5.]
                ]
            )
            .push(node!(
                IconButton::new("right_arrow_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::AddNetwork
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
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        content_node = content_node.push(toggle_node);
        content_node = content_node.push(node!(HDivider { size: 1. }));

        content_node = content_node.push(connected_network_row);
        content_node = content_node.push(node!(HDivider { size: 1. }));

        content_node = content_node.push(available_network_text);

        // TODO : HANDLE MORE

        content_node = content_node.push(node!(HDivider { size: 1. }));
        content_node = content_node.push(available_network_row_1);
        content_node = content_node.push(node!(HDivider { size: 0.5 }));
        content_node = content_node.push(available_network_row_2);
        content_node = content_node.push(node!(HDivider { size: 1. }));

        content_node = content_node.push(view_all_text);
        content_node = content_node.push(advanced_nextwork_text);

        content_node = content_node.push(node!(HDivider { size: 1. }));
        content_node = content_node.push(advanced_network_row);
        content_node = content_node.push(node!(HDivider { size: 1. }));

        base = base.push(header_node);
        base = base.push(content_node);
        Some(base)
    }
}
