pub mod identity_manager;
pub mod settings_manager;

use std::process::id;

use anyhow::{bail, Result};
use identity_manager::{GetMachineIdResponse, GetProvisionStatusResponse, IdentityClient};
use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use settings_manager::{GetSettingsResponse, SettingsClient};
use tokio::{runtime::Runtime, sync::oneshot::error};
use tonic::Response;
use uname::uname;

use crate::screens::network::wireless_model::network_manager;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
    static ref DEVICE_MODEL: DeviceModel = DeviceModel {
        is_provisioned: Context::new(false),
        os_info: Context::new(None),
        provision_name: Context::new("-".to_string()),
        provision_id: Context::new("-".to_string()),
        provision_icon_url: Context::new("-".to_string())
    };
}

#[derive(Debug, Clone)]
pub struct OSInfo {
    pub name: String,
    pub version: String,
}

#[derive(Model)]
pub struct DeviceModel {
    pub is_provisioned: Context<bool>,
    pub os_info: Context<Option<OSInfo>>,
    pub provision_name: Context<String>,
    pub provision_id: Context<String>,
    pub provision_icon_url: Context<String>,
}

impl DeviceModel {
    pub fn get() -> &'static Self {
        &DEVICE_MODEL
    }

    pub fn update() {
        RUNTIME.spawn(async {
            let status = match Self::check_provision_device_data().await {
                Ok(data) => data.status,
                Err(e) => {
                    println!("error while getting provision status: {}", e);
                    false
                }
            };

            DeviceModel::get().is_provisioned.set(status);

            if *DeviceModel::get().is_provisioned.get() {
                let machine_id = match Self::get_machine_id().await {
                    Ok(data) => data.machine_id,
                    Err(e) => {
                        println!("error while getting machine id: {}", e);
                        "-".to_string()
                    }
                };
                DeviceModel::get().provision_id.set(machine_id);

                let machine_name =
                    match Self::get_machine_info("identity.machine.name".to_string()).await {
                        Ok(data) => data.value,
                        Err(e) => {
                            println!("error while getting machine name: {}", e);
                            "-".to_string()
                        }
                    };
                DeviceModel::get().provision_name.set(machine_name);

                let machine_icon_url =
                    match Self::get_machine_info("identity.machine.icon_url".to_string()).await {
                        Ok(data) => data.value,
                        Err(e) => {
                            println!("error while getting machine name: {}", e);
                            "-".to_string()
                        }
                    };
                DeviceModel::get().provision_icon_url.set(machine_icon_url);
            }

            // os_info
            let info = uname().unwrap();
            let os_info = OSInfo {
                name: info.sysname.to_string(),
                version: info.release.to_string(),
            };
            DeviceModel::get().os_info.set(Some(os_info));
        });
    }

    pub async fn check_provision_device_data() -> Result<GetProvisionStatusResponse> {
        let handler = RUNTIME.spawn(async move {
            let mut service_client = match IdentityClient::new().await {
                Ok(response) => response,
                Err(error) => {
                    println!(
                        "check_provision_device_data::service_client::new::error::  {:?} ",
                        &error.to_string()
                    );
                    return Err(error);
                }
            };

            let response = match service_client.get_machine_provision_status().await {
                Ok(response) => {
                    response
                }
                Err(error) => {
                    println!(
                        "check_provision_device_data::service_client::get_machine_provision_status::error::  {:?} ",
                        &error.to_string()
                    );
                    return Err(error);
                }
            };

            Ok(response)

        });
        let result = handler.await.unwrap();
        return result;
    }

    pub async fn get_machine_id() -> Result<GetMachineIdResponse> {
        let handler = RUNTIME.spawn(async move {
            let mut service_client = match IdentityClient::new().await {
                Ok(response) => response,
                Err(error) => {
                    println!(
                        "get_machine_info::service_client::new::error::  {:?} ",
                        &error.to_string()
                    );
                    return Err(error);
                }
            };

            let response = match service_client.getting_machine_id().await {
                Ok(response) => response,
                Err(error) => {
                    println!(
                        "get_machine_info::service_client::getting_machine_id::error::  {:?} ",
                        &error.to_string()
                    );
                    return Err(error);
                }
            };

            Ok(response)
        });

        let result = handler.await.unwrap();
        return result;
    }

    pub async fn get_machine_info(key: String) -> Result<GetSettingsResponse> {
        let handler = RUNTIME.spawn(async move {
            let mut service_client = match SettingsClient::new().await {
                Ok(r) => r,
                Err(error) => {
                    println!(
                        "get_machine_info::service_client::new::error::  {:?} ",
                        &error.to_string()
                    );
                    return Err(error);
                }
            };

            let response = match service_client.get_settings_data(key.clone()).await {
                Ok(response) => response,
                Err(error) => {
                    println!(
                        "get_machine_info::service_client::get_settings_data::error::  {:?} ",
                        &error.to_string()
                    );
                    return Err(error);
                }
            };
            Ok(response)
        });

        let result = handler.await.unwrap();
        return result;
    }
}
