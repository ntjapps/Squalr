pub mod handler_settings_list_response;
pub mod handler_settings_set_response;

use crate::response_handlers::settings::handler_settings_list_response::handle_settings_list_response;
use crate::response_handlers::settings::handler_settings_set_response::handle_settings_set_response;
use squalr_engine_api::commands::settings::settings_response::SettingsResponse;

pub fn handle_settings_response(cmd: SettingsResponse) {
    match cmd {
        SettingsResponse::Memory { memory_settings_response } => handle_settings_list_response(memory_settings_response),
        SettingsResponse::Scan { scan_settings_response } => handle_settings_set_response(scan_settings_response),
    }
}
