mod memory;
mod process;
mod project;
mod project_items;
mod scan;
mod scan_results;
mod settings;
mod trackable_tasks;

use crate::response_handlers::memory::handle_memory_response;
use crate::response_handlers::process::handle_process_response;
use crate::response_handlers::project::handle_project_response;
use crate::response_handlers::project_items::handle_project_items_response;
use crate::response_handlers::scan::handle_scan_response;
use crate::response_handlers::scan_results::handle_scan_results_response;
use crate::response_handlers::settings::handle_settings_response;
use crate::response_handlers::trackable_tasks::handle_trackable_tasks_response;
use squalr_engine_api::commands::engine_command_response::EngineCommandResponse;

pub fn handle_engine_response(response: EngineCommandResponse) {
    match response {
        EngineCommandResponse::Scan(response) => handle_scan_response(response),
        EngineCommandResponse::Memory(response) => handle_memory_response(response),
        EngineCommandResponse::Process(response) => handle_process_response(response),
        EngineCommandResponse::Results(response) => handle_scan_results_response(response),
        EngineCommandResponse::Project(response) => handle_project_response(response),
        EngineCommandResponse::ProjectItems(response) => handle_project_items_response(response),
        EngineCommandResponse::Settings(response) => handle_settings_response(response),
        EngineCommandResponse::TrackableTasks(response) => handle_trackable_tasks_response(response),
    }
}
