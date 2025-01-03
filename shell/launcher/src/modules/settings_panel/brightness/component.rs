use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Div, Text, SlideBar, SlideBarType};
use mctk_core::{component, lay, msg, rect, size, size_pct, txt, Color};
use mctk_core::{component::Component, node, Node};
use std::hash::Hash;

use crate::gui;

#[derive(Debug)]
pub struct Brightness {
    pub value: u8,
}

impl Component for Brightness {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.value.hash(hasher);
    }
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new(),
                lay![direction: Direction::Column, cross_alignment:Alignment::Stretch, size_pct:[100, Auto]]
            )
            .push(node!(Text::new(txt!("BRIGHTNESS"))
                .with_class("text-white font-space-mono font-normal")
                .style("size", 15.0)
                ))
            .push(node!(
                SlideBar::new()
                .value(self.value)
                .slider_type(SlideBarType::Box)
                .active_color(Color::rgb(15.,168.,255.))
                .on_slide(Box::new(|value| msg!(gui::Message::SliderChanged(gui::SliderSettingsNames::Brightness { value }))))
                .col_spacing(7.75)
                .row_spacing(7.75)
                .col_width(4.), 
                lay![size: [Auto, 45], margin:[10., 0., 0., 0.]]
            ), )
        )
    }
}


