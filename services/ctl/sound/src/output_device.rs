use anyhow::{anyhow, Result};
use libpulse_binding::context::introspect::SourceInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use libpulse_binding::context::Context;
use libpulse_binding::mainloop::standard::Mainloop;

use libpulse_binding::volume::ChannelVolumes;

use crate::sound::{
    apply_volume_command, connect, run, volume_to_percentage, VolumeCommand, Volumes,
};

/// Run a volume command on the output device.
pub fn run_output_command(command: VolumeCommand, device: Option<String>) -> Result<()> {
    let mut main_loop = Mainloop::new()
        .ok_or_else(|| eprintln!("Failed to initialize PulseAudio main loop."))
        .unwrap();

    let mut context = connect(&mut main_loop).unwrap();

    let mut volumes =
        get_output_volumes(&mut main_loop, &mut context, device.clone()).map_err(|e| {
            anyhow!(
                "Error while getting output volumes in get_output_volumes : {:?} ",
                e
            )
        })?;

    if let VolumeCommand::Get = &command {
        let max = volume_to_percentage(volumes.channels.max());
        println!("{max:.0}");
        return Ok(());
    }

    apply_volume_command(&mut volumes, &command);

    set_output_volumes(
        &mut main_loop,
        &mut context,
        &volumes.channels,
        device.clone(),
    )
    .map_err(|e| anyhow!("Error in fn set_output_volumes : {:?}", e))?;

    set_output_muted(&mut main_loop, &mut context, volumes.muted, device.clone())
        .map_err(|e| anyhow!("Error in fn set_output_muted : {:?}", e))?;

    Ok(())
}

/// Get the volume of the output device.
pub fn get_output_volumes(
    main_loop: &mut Mainloop,
    context: &Context,
    device: Option<String>,
) -> Result<Volumes> {
    let device = match device {
        Some(device) if !device.trim().is_empty() => device,
        _ => "@DEFAULT_SINK@".to_string(),
    };

    match run(main_loop, move |output| {
        context
            .introspect()
            .get_sink_info_by_name(&device, move |info| match info {
                libpulse_binding::callbacks::ListResult::Item(x) => {
                    *output.lock().unwrap() = Some(Ok(Volumes {
                        muted: x.mute,
                        channels: x.volume,
                    }));
                }
                libpulse_binding::callbacks::ListResult::End => {}
                libpulse_binding::callbacks::ListResult::Error => {
                    *output.lock().unwrap() = Some(Err(()));
                }
            });
    }) {
        Ok(volumes) => Ok(volumes.unwrap()),
        Err(_) => Err(anyhow!("Error in fn run")),
    }
}

/// Get the volume of the output device.
pub fn set_output_volumes(
    main_loop: &mut Mainloop,
    context: &Context,
    volumes: &ChannelVolumes,
    device: Option<String>,
) -> Result<()> {
    let device = match device {
        Some(device) if !device.trim().is_empty() => device,
        _ => "@DEFAULT_SINK@".to_string(),
    };

    run(main_loop, move |output| {
        context.introspect().set_sink_volume_by_name(
            &device,
            volumes,
            Some(Box::new(move |success| {
                if success {
                    *output.lock().unwrap() = Some(Ok(()));
                } else {
                    *output.lock().unwrap() = Some(Err(()));
                }
            })),
        );
    })?
    .map_err(|_| anyhow!(context.errno()))
}

/// Set the muted state of the output device.
pub fn set_output_muted(
    main_loop: &mut Mainloop,
    context: &Context,
    muted: bool,
    device: Option<String>,
) -> Result<()> {
    let device = match device {
        Some(device) if !device.trim().is_empty() => device,
        _ => "@DEFAULT_SINK@".to_string(),
    };
    run(main_loop, move |output| {
        context.introspect().set_sink_mute_by_name(
            &device,
            muted,
            Some(Box::new(move |success| {
                if success {
                    *output.lock().unwrap() = Some(Ok(()));
                } else {
                    *output.lock().unwrap() = Some(Err(()));
                }
            })),
        );
    })?
    .map_err(|_| anyhow!(context.errno()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInformation {
    pub name: String,
    pub description: String,
    pub prop_list: HashMap<String, String>,
}

impl<'a> From<&SourceInfo<'a>> for SourceInformation {
    fn from(info: &SourceInfo) -> Self {
        let mut properties = HashMap::new();
        for key in info.proplist.iter() {
            if let Some(value) = info.proplist.get_str(&key) {
                properties.insert(key, value);
            }
        }

        Self {
            name: info.name.clone().unwrap_or_default().to_string(),
            description: info.description.clone().unwrap_or_default().to_string(),
            prop_list: properties,
        }
    }
}

pub fn get_connected_devices<'a>(
    main_loop: &mut Mainloop,
    context: &'a Context,
) -> Result<Vec<SourceInformation>> {
    let list_of_output_devices = Arc::new(Mutex::new(Vec::<SourceInformation>::new())); // Owned by the closure

    {
        let list_of_output_devices = Arc::clone(&list_of_output_devices);
        let _ = run(main_loop, move |output| {
            println!("Starting introspection");
            context.introspect().get_source_info_list(move |info| {
                println!("Inside get_source_output_info_list callback");
                match info {
                    libpulse_binding::callbacks::ListResult::Item(x) => {
                        let source_information = SourceInformation::from(x);
                        list_of_output_devices
                            .lock()
                            .unwrap()
                            .push(source_information);
                    }
                    libpulse_binding::callbacks::ListResult::End => {
                        *output.lock().unwrap() = Some(Ok::<_, ()>(()));
                    }
                    libpulse_binding::callbacks::ListResult::Error => {
                        eprintln!("Encountered an error while getting the device list");
                    }
                }
            });
            println!("Finished introspection");
        })?;
    }

    let list_of_output_devices = Arc::try_unwrap(list_of_output_devices)
        .map_err(|_| ())
        .unwrap()
        .into_inner()
        .map_err(|_| ())
        .unwrap();

    list_of_output_devices.iter().for_each(|device| {
        println!("--------------Output device----------------");
        println!("{:?}", device);
    });

    Ok(list_of_output_devices)
}