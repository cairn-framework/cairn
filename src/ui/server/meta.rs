//! Webui metadata response decoration.

use super::*;

impl Server {
    pub(super) fn meta(&self, project: &scanner::ScanResult) -> Response {
        match self.spine_data(project, "ui_meta", None, std::collections::BTreeSet::new()) {
            Ok(mut data) => {
                let last_reconciled = self.cached_scan_time.borrow().and_then(|time| {
                    time.duration_since(SystemTime::UNIX_EPOCH)
                        .ok()
                        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
                });
                data["last_reconciled"] =
                    last_reconciled.map_or(serde_json::Value::Null, serde_json::Value::from);
                json(200, &data.to_string())
            }
            Err(error) => error,
        }
    }
}
