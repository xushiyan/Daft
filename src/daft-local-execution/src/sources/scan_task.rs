use std::sync::Arc;

use async_trait::async_trait;
use daft_io::IOStatsContext;
use daft_micropartition::MicroPartition;
use daft_scan::ScanTask;
use futures::{stream, StreamExt};

use super::source::{Source, SourceStream};

pub struct ScanTaskSource {
    scan_tasks: Vec<Arc<ScanTask>>,
}

impl ScanTaskSource {
    pub fn new(scan_tasks: Vec<Arc<ScanTask>>) -> Self {
        Self { scan_tasks }
    }
}

#[async_trait]
impl Source for ScanTaskSource {
    async fn get_data(&self) -> SourceStream {
        log::debug!("ScanTaskSource::get_data");
        let stream = stream::iter(self.scan_tasks.clone().into_iter().map(|scan_task| {
            let io_stats = IOStatsContext::new("MicroPartition::from_scan_task");
            let out =
                std::thread::spawn(move || MicroPartition::from_scan_task(scan_task, io_stats))
                    .join()
                    .expect("Failed to join thread")?;

            // TODO: Implement dynamic splitting / merging of MicroPartition from scan task
            Ok(Arc::new(out))
        }));
        stream.boxed()
    }
}