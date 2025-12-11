pub mod handler_settings_list_response;
pub mod handler_settings_set_response;

use crate::response_handlers::settings::handler_settings_list_response::handle_settings_list_response;
use crate::response_handlers::settings::handler_settings_set_response::handle_settings_set_response;
use squalr_engine_api::commands::settings::settings_response::SettingsResponse;

pub fn handle_settings_response(cmd: SettingsResponse) {
    match cmd {
        SettingsResponse::Memory { memory_settings_response } => {
            match memory_settings_response {
                squalr_engine_api::commands::settings::memory::memory_settings_response::MemorySettingsResponse::List { memory_settings_list_response } => {
                    handle_settings_list_response(memory_settings_list_response);
                }
                squalr_engine_api::commands::settings::memory::memory_settings_response::MemorySettingsResponse::Set { .. } => {
                    // TODO: Implement memory settings set response handling
                }
            }
        }
        SettingsResponse::Scan { scan_settings_response } => {
            match scan_settings_response {
                squalr_engine_api::commands::settings::scan::scan_settings_response::ScanSettingsResponse::Set { scan_settings_set_response } => {
                    handle_settings_set_response(scan_settings_set_response);
                }
                squalr_engine_api::commands::settings::scan::scan_settings_response::ScanSettingsResponse::List { .. } => {
                    // TODO: Implement scan settings list response handling
                }
            }
        }
    }
}
