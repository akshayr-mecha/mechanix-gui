use mctk_core::layout::Alignment;
use mctk_core::widgets::Div;
use mctk_core::{component::Component, node, Node};
use mctk_core::{lay, msg, size, size_pct, Color, Point};

use crate::gui::{Message, Swipe, SwipeDirection, SwipeGestures, SwipeState};
#[derive(Debug)]
pub struct Closer {}

impl Closer {
    fn handle_on_drag(&self, delta: Point) -> Option<mctk_core::component::Message> {
        if delta.y.abs() > 10. {
            let swipe = Swipe {
                dy: (480. - 124. + delta.y) as i32,
                min_dy: 0,
                max_dy: 480 - 124,
                direction: SwipeDirection::Up,
                state: SwipeState::UserSwiping,
                is_closer: true,
                ..Default::default()
            };

            return Some(msg!(Message::Swipe { swipe }));
        }

        None
    }
}

impl Component for Closer {
    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Drag>) {
        println!("Closer::on_drag() {:?}", event.logical_delta());
        if let Some(msg) = self.handle_on_drag(event.logical_delta()) {
            event.emit(msg);
        };
    }
    fn on_drag_end(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragEnd>) {
        event.emit(msg!(Message::SwipeEnd));
    }

    fn on_touch_drag_start(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragStart>,
    ) {
        event.stop_bubbling();
    }

    fn on_touch_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::TouchDrag>) {
        println!("Closer::on_touch_drag() {:?}", event.logical_delta());
        if let Some(msg) = self.handle_on_drag(event.logical_delta()) {
            event.emit(msg);
        };
    }
    fn on_touch_drag_end(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragEnd>,
    ) {
        event.emit(msg!(Message::SwipeEnd));
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new(),
                lay![size_pct: [100], axis_alignment: Alignment::Center, cross_alignment: Alignment::Center]
            )
            .push(
                node!(Div::new().bg(Color::rgb(132.,132.,132.)), lay![size: [160, 2.5]]),
            ),
        )
    }
}
