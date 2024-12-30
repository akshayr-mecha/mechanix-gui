use std::hash::Hash;

use crate::contexts::camera;
use crate::contexts::state;
use mctk_core::prelude::*;
use mctk_core::widgets::RadioButtons;
use mctk_core::widgets::Scrollable;

#[derive(Debug)]
pub struct Settings;
impl Settings {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Settings {
    fn render_hash(&self, hasher: &mut ComponentHasher) {
        state::State::get_settings_state().hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let mut base = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size: size!(480.0, 300.0),
                position_type: Absolute,
                position: [180.0, Auto, Auto, -240.0],
                direction: Direction::Column,
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Center,
            ]
        );

        let mut header = node!(
            Div::new(),
            lay![
                size: size!(480.0, 50.0),
                direction: Direction::Row,
                axis_alignment: Alignment::End,
                cross_alignment: Alignment::Center,
            ]
        );
        let close_button = node!(
            Button::new(txt!("X"))
                .style("font_size", 20.0)
                .style("text_color", Color::WHITE)
                .style("background_color", Color::BLACK)
                .on_click(Box::new(|| {
                    state::State::set_settings_state(false);
                    msg!(())
                })),
            lay![
                size: size!(30.0, 30.0),
                margin: [10.0]
            ]
        );

        let heading = node!(
            Text::new(txt!("Select Capture Resolution"))
                .style("size", 20.0)
                .style("color", Color::WHITE),
            lay![margin: [0.0, 10.0, 0.0, 160.0]]
        );

        header = header.push(heading);
        header = header.push(close_button);
        let mut scrollable = node!(
            Scrollable::new(),
            lay![
                size: size!(480.0, 250.0),
                direction: Direction::Column,
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Start,
            ]
        );

        let resolutions = camera::Camera::get().compatible_resoultions.get();
        let mut index = 0;
        for (i, resolution) in resolutions.iter().enumerate() {
            if resolution.0.width_x == *camera::Camera::get().capture_width.get()
                && resolution.0.height_y == *camera::Camera::get().capture_height.get()
            {
                index = i;
            }
        }
        let selection = node!(
            RadioButtons::new(
                resolutions
                    .iter()
                    .map(|resolution| {
                        txt!(format!(
                            "{}x{}",
                            resolution.0.width_x, resolution.0.height_y
                        ))
                    })
                    .collect(),
                index,
            )
            .direction(mctk_core::layout::Direction::Column)
            .style("font_size", 18.0)
            .style("padding", 0.)
            .on_change(Box::new(|index| {
                let resolutions = camera::Camera::get().compatible_resoultions.get();
                let resolution = resolutions.iter().nth(index).unwrap();
                camera::Camera::get()
                    .capture_width
                    .set(resolution.0.width_x);
                camera::Camera::get()
                    .capture_height
                    .set(resolution.0.height_y);
                msg!(())
            }))
            .max_columns(1),
            lay![ size: [440, Auto], margin: [0.0, 20.0, 0.0, 0.0],]
        );
        scrollable = scrollable.push(selection);
        base = base.push(header);
        base = base.push(scrollable);
        Some(base)
    }
}
