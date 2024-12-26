//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `Interface '/org/freedesktop/NetworkManager' from service 'org.freedesktop.NetworkManager' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    /// ActivateConnection method
    fn activate_connection(
        &self,
        connection: &zbus::zvariant::ObjectPath<'_>,
        device: &zbus::zvariant::ObjectPath<'_>,
        specific_object: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// AddAndActivateConnection method
    fn add_and_activate_connection(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
        >,
        device: &zbus::zvariant::ObjectPath<'_>,
        specific_object: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<(
        zbus::zvariant::OwnedObjectPath,
        zbus::zvariant::OwnedObjectPath,
    )>;

    /// AddAndActivateConnection2 method
    #[allow(clippy::too_many_arguments)]
    fn add_and_activate_connection2(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
        >,
        device: &zbus::zvariant::ObjectPath<'_>,
        specific_object: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(
        zbus::zvariant::OwnedObjectPath,
        zbus::zvariant::OwnedObjectPath,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;

    /// CheckConnectivity method
    fn check_connectivity(&self) -> zbus::Result<u32>;

    /// CheckpointAdjustRollbackTimeout method
    fn checkpoint_adjust_rollback_timeout(
        &self,
        checkpoint: &zbus::zvariant::ObjectPath<'_>,
        add_timeout: u32,
    ) -> zbus::Result<()>;

    /// CheckpointCreate method
    fn checkpoint_create(
        &self,
        devices: &[&zbus::zvariant::ObjectPath<'_>],
        rollback_timeout: u32,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// CheckpointDestroy method
    fn checkpoint_destroy(&self, checkpoint: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// CheckpointRollback method
    fn checkpoint_rollback(
        &self,
        checkpoint: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<std::collections::HashMap<String, u32>>;

    /// DeactivateConnection method
    fn deactivate_connection(
        &self,
        active_connection: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// Enable method
    fn enable(&self, enable: bool) -> zbus::Result<()>;

    /// GetAllDevices method
    fn get_all_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GetDeviceByIpIface method
    fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetDevices method
    fn get_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GetLogging method
    fn get_logging(&self) -> zbus::Result<(String, String)>;

    /// GetPermissions method
    fn get_permissions(&self) -> zbus::Result<std::collections::HashMap<String, String>>;

    /// Reload method
    fn reload(&self, flags: u32) -> zbus::Result<()>;

    /// SetLogging method
    fn set_logging(&self, level: &str, domains: &str) -> zbus::Result<()>;

    /// Sleep method
    fn sleep(&self, sleep: bool) -> zbus::Result<()>;

    /// state method
    // #[zbus(name = "state")]
    // fn state(&self) -> zbus::Result<u32>;

    /// CheckPermissions signal
    #[zbus(signal)]
    fn check_permissions(&self) -> zbus::Result<()>;

    /// DeviceAdded signal
    #[zbus(signal)]
    fn device_added(&self, device_path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// DeviceRemoved signal
    #[zbus(signal)]
    fn device_removed(&self, device_path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// StateChanged signal
    // #[zbus(signal)]
    // fn state_changed(&self, state: u32) -> zbus::Result<()>;

    /// ActivatingConnection property
    #[zbus(property)]
    fn activating_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ActiveConnections property
    #[zbus(property)]
    fn active_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// AllDevices property
    #[zbus(property)]
    fn all_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Capabilities property
    #[zbus(property)]
    fn capabilities(&self) -> zbus::Result<Vec<u32>>;

    /// Checkpoints property
    #[zbus(property)]
    fn checkpoints(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Connectivity property
    #[zbus(property)]
    fn connectivity(&self) -> zbus::Result<u32>;

    /// ConnectivityCheckAvailable property
    #[zbus(property)]
    fn connectivity_check_available(&self) -> zbus::Result<bool>;

    /// ConnectivityCheckEnabled property
    #[zbus(property)]
    fn connectivity_check_enabled(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_connectivity_check_enabled(&self, value: bool) -> zbus::Result<()>;

    /// ConnectivityCheckUri property
    #[zbus(property)]
    fn connectivity_check_uri(&self) -> zbus::Result<String>;

    /// Devices property
    #[zbus(property)]
    fn devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GlobalDnsConfiguration property
    #[zbus(property)]
    fn global_dns_configuration(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
    // #[zbus(property)]
    // fn set_global_dns_configuration(
    //     &self,
    //     value: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    // ) -> zbus::Result<()>;

    /// Metered property
    #[zbus(property)]
    fn metered(&self) -> zbus::Result<u32>;

    /// NetworkingEnabled property
    #[zbus(property)]
    fn networking_enabled(&self) -> zbus::Result<bool>;

    /// PrimaryConnection property
    #[zbus(property)]
    fn primary_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// PrimaryConnectionType property
    #[zbus(property)]
    fn primary_connection_type(&self) -> zbus::Result<String>;

    /// RadioFlags property
    #[zbus(property)]
    fn radio_flags(&self) -> zbus::Result<u32>;

    /// Startup property
    #[zbus(property)]
    fn startup(&self) -> zbus::Result<bool>;

    /// State property
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    /// Version property
    #[zbus(property)]
    fn version(&self) -> zbus::Result<String>;

    /// VersionInfo property
    #[zbus(property)]
    fn version_info(&self) -> zbus::Result<Vec<u32>>;

    /// WimaxEnabled property
    #[zbus(property)]
    fn wimax_enabled(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_wimax_enabled(&self, value: bool) -> zbus::Result<()>;

    /// WimaxHardwareEnabled property
    #[zbus(property)]
    fn wimax_hardware_enabled(&self) -> zbus::Result<bool>;

    /// WirelessEnabled property
    #[zbus(property)]
    fn wireless_enabled(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_wireless_enabled(&self, value: bool) -> zbus::Result<()>;

    /// WirelessHardwareEnabled property
    #[zbus(property)]
    fn wireless_hardware_enabled(&self) -> zbus::Result<bool>;

    /// WwanEnabled property
    #[zbus(property)]
    fn wwan_enabled(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_wwan_enabled(&self, value: bool) -> zbus::Result<()>;

    /// WwanHardwareEnabled property
    #[zbus(property)]
    fn wwan_hardware_enabled(&self) -> zbus::Result<bool>;
}