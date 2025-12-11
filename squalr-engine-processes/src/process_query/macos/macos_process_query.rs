use crate::process_query::process_query_options::ProcessQueryOptions;
use crate::process_query::process_queryer::ProcessQueryer;
use squalr_engine_api::structures::processes::opened_process_info::OpenedProcessInfo;
use squalr_engine_api::structures::processes::process_icon::ProcessIcon;
use squalr_engine_api::structures::processes::process_info::ProcessInfo;
use sysinfo::{Pid, System};

pub struct MacosProcessQuery {
    #[allow(dead_code)]
    system: System,
}

impl MacosProcessQuery {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MacosProcessQuery { system: System::new_all() }
    }

    #[allow(dead_code)]
    fn is_process_windowed(_process_id: &Pid) -> bool {
        false
    }

    #[allow(dead_code)]
    fn get_icon(_process_id: &Pid) -> Option<ProcessIcon> {
        None
    }
}

impl ProcessQueryer for MacosProcessQuery {
    fn start_monitoring() -> Result<(), String> {
        Ok(())
    }

    fn stop_monitoring() -> Result<(), String> {
        Ok(())
    }

    fn open_process(_process_info: &ProcessInfo) -> Result<OpenedProcessInfo, String> {
        Err("Not implemented".into())
    }

    fn close_process(_handle: u64) -> Result<(), String> {
        Err("Not implemented".into())
    }

    fn get_processes(_options: ProcessQueryOptions) -> Vec<ProcessInfo> {
        vec![]
    }
}
