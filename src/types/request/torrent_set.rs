use super::{TorrentSetArgs, Id, Priority, TrackerList};

impl TorrentSetArgs {
    /// Creates a new [`TorrentSetArgs`] with all fields set to `None`.
    pub fn new() -> Self { Self::default() }

    pub fn bandwidth_priority(self, bandwidth_priority: Priority) -> Self {
        Self { bandwidth_priority: Some(bandwidth_priority), ..self }
    }
    pub fn download_limit(self, download_limit: i32) -> Self {
        Self { download_limit: Some(download_limit), ..self }
    }
    pub fn download_limited(self, download_limited: bool) -> Self {
        Self { download_limited: Some(download_limited), ..self }
    }
    pub fn files_wanted(self, files_wanted: Vec<i32>) -> Self {
        Self { files_wanted: Some(files_wanted), ..self }
    }
    pub fn files_unwanted(self, files_unwanted: Vec<i32>) -> Self {
        Self { files_unwanted: Some(files_unwanted), ..self }
    }
    pub fn honors_session_limits(self, honors_session_limits: bool) -> Self {
        Self { honors_session_limits: Some(honors_session_limits), ..self }
    }

    pub fn ids(self, ids: Vec<Id>) -> Self {
        Self { ids: Some(ids), ..self }
    }

    pub fn labels(self, labels: Vec<String>) -> Self {
        Self { labels: Some(labels), ..self }
    }
    pub fn location(self, location: String) -> Self {
        Self { location: Some(location), ..self }
    }
    pub fn peer_limit(self, peer_limit: i64) -> Self {
        Self { peer_limit: Some(peer_limit), ..self }
    }
    pub fn priority_high(self, priority_high: Vec<i32>) -> Self {
        Self { priority_high: Some(priority_high), ..self }
    }
    pub fn priority_low(self, priority_low: Vec<i32>) -> Self {
        Self { priority_low: Some(priority_low), ..self }
    }
    pub fn priority_normal(self, priority_normal: Vec<i32>) -> Self {
        Self { priority_normal: Some(priority_normal), ..self }
    }
    pub fn queue_position(self, queue_position: i32) -> Self {
        Self { queue_position: Some(queue_position), ..self }
    }
    pub fn seed_idle_limit(self, seed_idle_limit: i32) -> Self {
        Self { seed_idle_limit: Some(seed_idle_limit), ..self }
    }
    pub fn seed_idle_mode(self, seed_idle_mode: i32) -> Self {
        Self { seed_idle_mode: Some(seed_idle_mode), ..self }
    }
    pub fn seed_ratio_limit(self, seed_ratio_limit: f32) -> Self {
        Self { seed_ratio_limit: Some(seed_ratio_limit), ..self }
    }
    pub fn seed_ratio_mode(self, seed_ratio_mode: i32) -> Self {
        Self { seed_ratio_mode: Some(seed_ratio_mode), ..self }
    }
    pub fn tracker_add(self, tracker_add: Vec<String>) -> Self {
        Self { tracker_add: Some(tracker_add), ..self }
    }
    pub fn tracker_list(self, tracker_list: TrackerList) -> Self {
        Self { tracker_list: Some(tracker_list), ..self }
    }
    pub fn tracker_remove(self, tracker_remove: Vec<String>) -> Self {
        Self { tracker_remove: Some(tracker_remove), ..self }
    }
    pub fn tracker_replace(self, tracker_replace: Vec<String>) -> Self {
        Self { tracker_replace: Some(tracker_replace), ..self }
    }
    pub fn upload_limit(self, upload_limit: i32) -> Self {
        Self { upload_limit: Some(upload_limit), ..self }
    }
    pub fn upload_limited(self, upload_limited: bool) -> Self {
        Self { upload_limited: Some(upload_limited), ..self }
    }
}
