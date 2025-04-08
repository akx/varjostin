use std::path::Path;
use std::time::{Instant, SystemTime};

#[derive(Debug)]
pub(crate) struct FileChangeState {
    checked_at: Instant,
    last_modified: SystemTime,
    size: u64,
}

impl FileChangeState {
    fn state_equal(&self, other: &Self) -> bool {
        self.last_modified == other.last_modified && self.size == other.size
    }
}

pub fn has_changed(
    file_path: &Path,
    last_state: &Option<FileChangeState>,
    min_check_interval: std::time::Duration,
) -> eyre::Result<Option<FileChangeState>> {
    if let Some(s) = last_state {
        if s.checked_at.elapsed() < min_check_interval {
            return Ok(None); // Too soon to check again
        }
    }
    let stat = file_path.metadata()?;
    let new_state = FileChangeState {
        checked_at: Instant::now(),
        last_modified: stat.modified()?,
        size: stat.len(),
    };
    if let Some(last_state) = last_state {
        if new_state.state_equal(last_state) {
            return Ok(None); // No change
        }
    }
    Ok(Some(new_state))
}
