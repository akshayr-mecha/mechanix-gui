use bevy::{
    platform::collections::{HashMap, HashSet},
    prelude::*,
};

mod components;
mod systems;
mod ui;

use components::{WidgetManagerPlugin, WingWidgetPlugin};
use systems::*;

pub struct HomescreenPlugin;
impl Plugin for HomescreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WingWidgetPlugin,));
        app.add_systems(Startup, (setup, setup_widgets).chain());
        app.add_systems(Update, (exit_on_esc, render_widgets));
        app.init_resource::<HomescreenData>();
    }
}

#[derive(Component, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct WidgetId(pub usize);

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct HomescreenWidget {
    pub id: WidgetId,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct HomescreenWidgetData {
    pub widget: HomescreenWidget,
    pub grid_location: UVec2,
    pub homescreen_index: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct WidgetRenderData {
    pub data: HomescreenWidgetData,
    pub screen_location: Vec2,
    pub screen_size: Vec2,
}

#[derive(Debug, Clone)]
struct HoverState {
    pub hovering_widget: HomescreenWidget,
    pub target_grid_location: UVec2,
    pub page: usize,
    pub displaced_widgets: HashMap<WidgetId, UVec2>,
}

#[derive(Debug, Resource)]
pub struct HomescreenData {
    pub screen_origin: Vec2,
    pub page_size: Vec2,
    pub rows: usize,
    pub cols: usize,
    pub number_of_screens: usize,
    pub active_screen: usize,
    widgets: HashMap<WidgetId, HomescreenWidgetData>,
    hover_state: Option<HoverState>,
}

impl HomescreenData {
    pub fn new(
        screen_origin: Vec2,
        page_size: Vec2,
        rows: usize,
        cols: usize,
        number_of_screens: usize,
    ) -> Self {
        Self {
            screen_origin,
            page_size,
            rows,
            cols,
            number_of_screens,
            active_screen: 0,
            widgets: HashMap::new(),
            hover_state: None,
        }
    }

    /// Commits a widget's position to the grid. This will displace other widgets
    /// as necessary to make space.
    /// Returns the final screen position of the placed widget, or None if it cannot be placed.
    pub fn place(&mut self, widget: HomescreenWidget, location: Vec2, page: usize) -> Option<Vec2> {
        let target_grid_location = self.screen_to_grid(location, page);

        if let Some((final_pos, displaced)) =
            self.calculate_placement(page, widget, target_grid_location)
        {
            // Commit the changes for displaced widgets
            for (id, new_grid_pos) in displaced {
                if let Some(data) = self.widgets.get_mut(&id) {
                    data.grid_location = new_grid_pos;
                }
            }

            // Add or update the main widget being placed
            let new_data = HomescreenWidgetData {
                widget,
                grid_location: final_pos,
                homescreen_index: page,
            };
            self.widgets.insert(widget.id, new_data);

            self.clear_hover();
            return Some(self.grid_to_screen(final_pos, page));
        }

        // Placement was not possible
        self.clear_hover();
        None
    }

    /// Simulates placing a widget at a given location, calculating where other
    /// widgets would be displaced to. The results are stored in a temporary hover state.
    pub fn hover(&mut self, widget: HomescreenWidget, location: Vec2, page: usize) {
        let target_grid_location = self.screen_to_grid(location, page);

        if let Some((final_pos, displaced)) =
            self.calculate_placement(page, widget, target_grid_location)
        {
            self.hover_state = Some(HoverState {
                hovering_widget: widget,
                target_grid_location: final_pos,
                page,
                displaced_widgets: displaced,
            });
        } else {
            self.hover_state = None;
        }
    }

    /// Clears any temporary hover state.
    pub fn clear_hover(&mut self) {
        self.hover_state = None;
    }

    /// Removes a widget from the homescreen entirely.
    pub fn remove(&mut self, widget_id: WidgetId) {
        self.widgets.remove(&widget_id);
    }

    /// Gets all data required to render the widgets on a specific page,
    /// accounting for any temporary hover state.
    pub fn get_page_render_data(&self, page: usize) -> Vec<WidgetRenderData> {
        let mut final_states: HashMap<WidgetId, HomescreenWidgetData> = HashMap::new();
        let cell_size = self.get_cell_size();

        // 1. Populate with the base state for the requested page
        for (id, data) in &self.widgets {
            if data.homescreen_index == page {
                final_states.insert(*id, *data);
            }
        }

        // 2. If a hover state is active on this page, apply its changes
        if let Some(hover_state) = &self.hover_state {
            if hover_state.page == page {
                // Update or insert the widget currently being hovered
                final_states.insert(
                    hover_state.hovering_widget.id,
                    HomescreenWidgetData {
                        widget: hover_state.hovering_widget,
                        grid_location: hover_state.target_grid_location,
                        homescreen_index: page,
                    },
                );

                // Update the positions of all displaced widgets
                for (id, new_pos) in &hover_state.displaced_widgets {
                    if let Some(data) = final_states.get_mut(id) {
                        data.grid_location = *new_pos;
                    }
                }
            }
        }

        // 3. Convert the final calculated states into render data
        final_states
            .values()
            .map(|data| WidgetRenderData {
                data: *data,
                screen_location: self.grid_to_screen(data.grid_location, page),
                screen_size: Vec2::new(
                    data.widget.width as f32 * cell_size.x,
                    data.widget.height as f32 * cell_size.y,
                ),
            })
            .collect()
    }

    /// Gets the stored data for a single widget.
    pub fn get_widget_data(&self, widget_id: WidgetId) -> Option<HomescreenWidgetData> {
        self.widgets.get(&widget_id).copied()
    }

    // --- Private Helper Functions ---

    /// The core logic for arranging widgets. It places the `moved_widget` at its
    /// target location, then reflows all other widgets into the first available
    /// spots. Returns the final position of the `moved_widget` and a map of
    /// any widgets that were displaced.
    fn calculate_placement(
        &self,
        page: usize,
        moved_widget: HomescreenWidget,
        target_grid_location: UVec2,
    ) -> Option<(UVec2, HashMap<WidgetId, UVec2>)> {
        // 1. Clamp target location to ensure the widget fits within the grid bounds
        let clamped_x = target_grid_location
            .x
            .min((self.cols.saturating_sub(moved_widget.width)) as u32);
        let clamped_y = target_grid_location
            .y
            .min((self.rows.saturating_sub(moved_widget.height)) as u32);
        let final_target_pos = UVec2::new(clamped_x, clamped_y);

        // 2. Get all other widgets on the page, sorted by their original position
        let mut other_widgets: Vec<_> = self
            .widgets
            .values()
            .filter(|data| data.homescreen_index == page && data.widget.id != moved_widget.id)
            .collect();
        other_widgets.sort_by_key(|data| (data.grid_location.y, data.grid_location.x));

        // 3. Create a temporary grid and place the moved widget first
        let mut temp_grid = vec![vec![None; self.cols]; self.rows];
        for y in final_target_pos.y as usize..(final_target_pos.y as usize + moved_widget.height) {
            for x in final_target_pos.x as usize..(final_target_pos.x as usize + moved_widget.width)
            {
                temp_grid[y][x] = Some(moved_widget.id);
            }
        }

        // 4. Iterate through all other widgets and place them in the first available spot
        let mut displaced_widgets = HashMap::new();
        for widget_data in other_widgets {
            let new_pos = Self::find_first_available_spot_static(
                &temp_grid,
                self.cols,
                self.rows,
                widget_data.widget,
            )?; // Use '?' to exit if any widget cannot be placed

            if new_pos != widget_data.grid_location {
                displaced_widgets.insert(widget_data.widget.id, new_pos);
            }

            // Occupy the new spot in the temp grid for the next widget
            for y in new_pos.y as usize..(new_pos.y as usize + widget_data.widget.height) {
                for x in new_pos.x as usize..(new_pos.x as usize + widget_data.widget.width) {
                    temp_grid[y][x] = Some(widget_data.widget.id);
                }
            }
        }

        Some((final_target_pos, displaced_widgets))
    }

    fn get_page_top_left(&self, page: usize) -> Vec2 {
        self.screen_origin + Vec2::new(self.page_size.x * page as f32, 0.0)
    }

    fn get_cell_size(&self) -> Vec2 {
        Vec2::new(
            self.page_size.x / self.cols as f32,
            self.page_size.y / self.rows as f32,
        )
    }

    fn screen_to_grid(&self, screen_pos: Vec2, page: usize) -> UVec2 {
        let page_origin = self.get_page_top_left(page);
        let cell_size = self.get_cell_size();

        if cell_size.x == 0.0 || cell_size.y == 0.0 {
            return UVec2::ZERO;
        }

        let relative_pos = screen_pos - page_origin;
        let grid_x = (relative_pos.x / cell_size.x).floor() as i32;
        let grid_y = (relative_pos.y / cell_size.y).floor() as i32;

        UVec2::new(
            (grid_x.clamp(0, self.cols as i32 - 1)) as u32,
            (grid_y.clamp(0, self.rows as i32 - 1)) as u32,
        )
    }

    fn grid_to_screen(&self, grid_pos: UVec2, page: usize) -> Vec2 {
        let page_origin = self.get_page_top_left(page);
        let cell_size = self.get_cell_size();
        let relative_pos = Vec2::new(
            grid_pos.x as f32 * cell_size.x,
            grid_pos.y as f32 * cell_size.y,
        );
        page_origin + relative_pos
    }

    // --- Static Helper Functions ---

    /// Checks if a widget can fit in the given occupancy grid at a specific position.
    fn is_space_free_static(
        occupancy_grid: &Vec<Vec<Option<WidgetId>>>,
        cols: usize,
        rows: usize,
        pos: UVec2,
        widget: HomescreenWidget,
    ) -> bool {
        if pos.x as usize + widget.width > cols || pos.y as usize + widget.height > rows {
            return false;
        }
        for y in pos.y as usize..(pos.y as usize + widget.height) {
            for x in pos.x as usize..(pos.x as usize + widget.width) {
                if occupancy_grid[y][x].is_some() {
                    return false;
                }
            }
        }
        true
    }

    /// Scans an occupancy grid from top-left to bottom-right to find the first
    /// available space that can accommodate the given widget.
    fn find_first_available_spot_static(
        occupancy_grid: &Vec<Vec<Option<WidgetId>>>,
        cols: usize,
        rows: usize,
        widget: HomescreenWidget,
    ) -> Option<UVec2> {
        if widget.height > rows || widget.width > cols {
            return None;
        }
        for y in 0..=(rows - widget.height) {
            for x in 0..=(cols - widget.width) {
                let pos = UVec2::new(x as u32, y as u32);
                if Self::is_space_free_static(occupancy_grid, cols, rows, pos, widget) {
                    return Some(pos);
                }
            }
        }
        None
    }
}

impl Default for HomescreenData {
    fn default() -> Self {
        Self {
            screen_origin: (0.0, 0.0).into(),
            page_size: (400.0, 400.0).into(),
            rows: 4,
            cols: 4,
            number_of_screens: 1,
            active_screen: 0,
            widgets: Default::default(),
            hover_state: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct HomescreenCamera;
pub mod prelude {
    pub use crate::HomescreenPlugin;
}
