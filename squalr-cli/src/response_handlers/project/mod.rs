pub mod handler_project_list_response;

use crate::response_handlers::project::handler_project_list_response::handle_project_list_response;
use squalr_engine_api::commands::project::project_response::ProjectResponse;

pub fn handle_project_response(cmd: ProjectResponse) {
    match cmd {
        ProjectResponse::List { project_list_response } => handle_project_list_response(project_list_response),
        ProjectResponse::Create { .. } => {
            // TODO: Implement project create response handling
        }
        ProjectResponse::Open { .. } => {
            // TODO: Implement project open response handling
        }
        ProjectResponse::Close { .. } => {
            // TODO: Implement project close response handling
        }
        ProjectResponse::Rename { .. } => {
            // TODO: Implement project rename response handling
        }
        ProjectResponse::Save { .. } => {
            // TODO: Implement project save response handling
        }
        ProjectResponse::Export { .. } => {
            // TODO: Implement project export response handling
        }
    }
}
