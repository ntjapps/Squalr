use crate::memory_writer::memory_writer_trait::IMemoryWriter;
use squalr_engine_api::structures::processes::opened_process_info::OpenedProcessInfo;

pub struct LinuxMemoryWriter;

impl LinuxMemoryWriter {
    pub fn new() -> Self {
        LinuxMemoryWriter
    }
}

impl IMemoryWriter for LinuxMemoryWriter {
    fn write_bytes(
        &self,
        _process_info: &OpenedProcessInfo,
        _address: u64,
        _values: &[u8],
    ) -> bool {
        false
    }
}
