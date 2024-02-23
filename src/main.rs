use anyhow::{bail, Result};
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::prelude::Peripherals};
use wifi::wifi;

pub struct Config {
    wifi_ssid: &'static str,
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let app_config = Config {
        wifi_ssid: "wifi",
        wifi_psk: "password"
    };
    // Connect to the Wi-Fi network
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            inner
        }
        Err(err) => {
            // Red!
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };
    Ok(())
}
