use std::collections::HashSet;
use std::fs::{self, read_dir, File};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::services::app_manager::{AppManagerMessage, AppManagerService};
use anyhow::{bail, Result};
use desktop_ini_utils::get_desktop_entries;
use gtk::{gdk, gio, glib, prelude::*, subclass::*};
use relm4::component::{AsyncComponent, AsyncComponentParts};
use relm4::gtk::GestureClick;
use relm4::{
    async_trait::async_trait,
    gtk::{
        glib::clone,
        prelude::{EditableExt, EditableExtManual, EntryExt, ObjectExt},
    },
    RelmRemoveAllExt,
};
use relm4::{
    gtk, AsyncComponentSender, Component, Controller, RelmApp, RelmWidgetExt, SimpleComponent,
};

use std::str::FromStr;
use tokio::sync::oneshot;
use tokio::{sync::mpsc, task};
use tonic::Status;

mod settings;
mod theme;
use settings::App;
use tracing::{error, info};
pub mod errors;
mod services;

use crate::errors::{HomescreenError, HomescreenErrorCodes};
use crate::settings::HomescreenSettings;
use crate::theme::HomescreenTheme;
use serde::{de, Deserialize, Deserializer, Serialize};

/// # Homescreen State
///
/// This struct is the state definition of the entire application
struct Homescreen {
    settings: HomescreenSettings,
    custom_theme: HomescreenTheme,
    search_text: String,
    apps: Vec<App>,
    filtered_apps: Vec<App>,
    app_manager_sender: mpsc::Sender<AppManagerMessage>,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    SearchTextChanged(String),
    AppClicked(String, String),
}

struct AppWidgets {
    apps_grid: gtk::FlowBox,
}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: HomescreenSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

#[cfg(feature = "layer-shell")]
fn init_window(settings: HomescreenSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();

    gtk4_layer_shell::init_for_window(&window);

    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Bottom);

    gtk4_layer_shell::set_keyboard_mode(&window, gtk4_layer_shell::KeyboardMode::OnDemand);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk4_layer_shell::Edge::Left, true),
        (gtk4_layer_shell::Edge::Right, true),
        (gtk4_layer_shell::Edge::Top, true),
        (gtk4_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
    }

    window
}

#[async_trait(?Send)]
impl AsyncComponent for Homescreen {
    /// The type of the messages that this component can receive.
    type Input = Message;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    type CommandOutput = Message;

    fn init_root() -> Self::Root {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => HomescreenSettings::default(),
        };

        info!(
            task = "init_settings",
            "settings initialized for homescreen {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => HomescreenTheme::default(),
        };

        info!(
            task = "init_theme",
            "theme initialized for homescreen {:?}", custom_theme
        );

        let window = init_window(settings);

        window
    }

    /// Initialize the UI and model.
    async fn init(
        _: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => HomescreenSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => HomescreenTheme::default(),
        };

        let modules = settings.modules.clone();

        let container_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .css_classes(["container"])
            .build();

        let apps_grid = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .max_children_per_line(30)
            .min_children_per_line(4)
            .selection_mode(gtk::SelectionMode::None)
            .row_spacing(10)
            .build();

        let desktop_entries = get_desktop_entries("/usr/share/applications");

        let apps: Vec<App> = desktop_entries[0..10]
            .into_iter()
            .map(|app| {
                let app_id = match app.exec.clone() {
                    Some(v) => String::from(v.split(" ").into_iter().nth(0).unwrap_or("")),
                    None => "".to_string(),
                };
                let app_name = match app.name.clone() {
                    Some(v) => v,
                    None => "".to_string(),
                };
                return App {
                    app_id: app_id,
                    name: app_name,
                    start_command: app.exec.clone(),
                    icon: app.icon.clone(),
                };
            })
            .collect();

        apps.iter().for_each(|app| {
            let widget = generate_apps_ui(app, sender.input_sender().clone());
            apps_grid.insert(&widget, -1);
        });

        let scrolled_window = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(360)
            .min_content_height(360)
            .css_classes(["scrollable"])
            .child(&apps_grid)
            .build();

        container_box.append(&scrolled_window);
        container_box.set_focus_child(Option::from(&scrolled_window));

        window.set_child(Some(&container_box));

        let (app_manager_sender) = init_services().await;

        let model = Homescreen {
            settings: settings.clone(),
            custom_theme,
            search_text: String::from(""),
            apps: apps.clone(),
            filtered_apps: apps.clone(),
            app_manager_sender,
        };

        let widgets = AppWidgets { apps_grid };

        AsyncComponentParts { model, widgets }
    }
    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        info!("Update message is {:?}", message);
        match message {
            Message::SearchTextChanged(term) => {
                self.search_text = term;
                self.filtered_apps = self
                    .apps
                    .clone()
                    .into_iter()
                    .filter(|app| app.name.to_lowercase().starts_with(&self.search_text))
                    .collect();
            }
            Message::AppClicked(app_id, start_command) => {
                let (tx, rx) = oneshot::channel();

                let _ = self
                    .app_manager_sender
                    .send(AppManagerMessage::LaunchApp {
                        app_id: app_id.clone(),
                        start_command,
                        reply_to: tx,
                    })
                    .await;

                let reply = rx
                    .await
                    .unwrap_or(Err(Status::unavailable("app manager unavailable").into()));

                let app_launched = match reply {
                    Ok(v) => v,
                    Err(e) => {
                        error!("error while launching app {}", e);
                        false
                    }
                };

                info!("app_launched is  {}", app_launched);

                // if !is_app_open {
                //     let app_op = self.apps.iter().find(|app| app.app_id == app_id.as_str());
                //     match app_op {
                //         Some(app) => match &app.start_command {
                //             Some(start_command) => {
                //                 let main_command: Vec<&str> = start_command.split(" ").collect();
                //                 let args: Vec<&str> = main_command.clone()[1..]
                //                     .iter()
                //                     .filter(|&&arg| arg != "%u" && arg != "%U" && arg != "%F")
                //                     .cloned()
                //                     .collect();
                //                 match spawn_command(main_command[0], &args) {
                //                     Ok(_) => {
                //                         info!("app started successfully {}", app_id);
                //                     }
                //                     Err(e) => {
                //                         error!("error while starting app app_id {} command {} error {}", app_id, start_command, e)
                //                     }
                //                 };
                //             }
                //             None => {
                //                 error!("Message::AppClicked start command not found for app with app_id {}", app_id)
                //             }
                //         },
                //         None => {
                //             error!("Message::AppClicked app not found with app_id {}", app_id)
                //         }
                //     }
                // } else {
                //     //Send event to open existing one
                //     let (tx, rx) = oneshot::channel();

                //     let _ = self
                //         .top_level_service_sender
                //         .send(AppManagerMessage::ActivateApp {
                //             app_id,
                //             reply_to: tx,
                //         })
                //         .await;

                //     let reply = rx
                //         .await
                //         .unwrap_or(Err(
                //             Status::unavailable("top level service unavailable").into()
                //         ));

                //     let activated_app = match reply {
                //         Ok(v) => v,
                //         Err(e) => {
                //             error!("error while getting activating app {}", e);
                //             false
                //         }
                //     };
                // }
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        widgets.apps_grid.remove_all();
        self.filtered_apps.iter().for_each(|app| {
            let widget = generate_apps_ui(&app, sender.input_sender().clone());
            widgets.apps_grid.insert(&widget, -1);
        });
    }
}

/// Initialize the application with settings, and starts
fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_homescreen=trace")
        .with_thread_names(true)
        .init();

    let app = RelmApp::new("homescreen").with_args(vec![]);
    app.run_async::<Homescreen>(());
}

fn generate_apps_ui(app: &App, sender: relm4::Sender<Message>) -> gtk::Box {
    let max_lenth = 15;
    let max_len_app_name = match app.name.len() > max_lenth {
        true => max_lenth,
        false => app.name.len(),
    };
    let app_name = &app.name[0..max_len_app_name];
    let app_name_label = gtk::Label::builder()
        .label(app_name)
        .wrap(true)
        .css_classes(["app-name-label"])
        .build();

    let mut app_icon = gtk::Image::builder()
        // .paintable(&app_icon_paintable)
        .css_classes(["app-image"])
        .icon_size(gtk::IconSize::Large)
        .pixel_size(88)
        .build();

    match &app.icon {
        Some(icon) => {
            app_icon.set_icon_name(Some(&icon));
        }
        None => {}
    }

    let app_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["app"])
        .build();
    app_box.append(&app_icon);
    app_box.append(&app_name_label);

    let left_click_gesture = GestureClick::builder().build();
    // left_click_gesture.connect_pressed(clone!(@strong sender => move |this, _, _,_| {
    // info!("gesture button pressed is {}", this.current_button());
    //     sender.input_sender().send(Me::Pressed);

    // }));

    left_click_gesture.connect_released(clone!(@strong app => move |this, _, _,_| {
            info!("gesture button released is {}", this.current_button());
            info!("app_is is {}", app.app_id);
            let _ = sender.send(Message::AppClicked(app.app_id.clone(), app.start_command.clone().unwrap_or("".to_string())));

    }));
    app_box.add_controller(left_click_gesture);
    app_box
}

async fn init_services() -> (mpsc::Sender<AppManagerMessage>) {
    let (app_manager_t, app_manager_tx) = init_app_manager().await;

    (app_manager_tx)
}

async fn init_app_manager() -> (glib::JoinHandle<()>, mpsc::Sender<AppManagerMessage>) {
    let (tx, rx) = mpsc::channel(32);

    let t = relm4::spawn_local(async move { AppManagerService::new().run(rx).await });

    (t, tx)
}
