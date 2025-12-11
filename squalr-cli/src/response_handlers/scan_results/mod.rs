pub mod handler_scan_results_list_response;

use crate::response_handlers::scan_results::handler_scan_results_list_response::handle_scan_results_list_response;
use squalr_engine_api::commands::scan_results::scan_results_response::ScanResultsResponse;

pub fn handle_scan_results_response(cmd: ScanResultsResponse) {
    match cmd {
        ScanResultsResponse::List { scan_results_list_response } => handle_scan_results_list_response(scan_results_list_response),
        ScanResultsResponse::Query { .. } => {
            // TODO: Implement scan results query response handling
        }
        ScanResultsResponse::Refresh { .. } => {
            // TODO: Implement scan results refresh response handling
        }
        ScanResultsResponse::AddToProject { .. } => {
            // TODO: Implement scan results add to project response handling
        }
        ScanResultsResponse::Freeze { .. } => {
            // TODO: Implement scan results freeze response handling
        }
        ScanResultsResponse::SetProperty { .. } => {
            // TODO: Implement scan results set property response handling
        }
        ScanResultsResponse::Delete { .. } => {
            // TODO: Implement scan results delete response handling
        }
    }
}
