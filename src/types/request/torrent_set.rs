use super::{Id, TorrentSetArgs, IdleMode, Priority, RatioMode, TrackerList};

impl TorrentSetArgs {
    /// Creates a new [`TorrentSetArgs`] with all fields set to `None`.
    pub fn new() -> Self { Self::default() }

    pub fn bandwidth_priority(mut self, bandwidth_priority: Priority) -> Self {
        self.bandwidth_priority = Some(bandwidth_priority);
        self
    }
    pub fn download_limit(mut self, download_limit: usize) -> Self {
        self.download_limit = Some(download_limit);
        self
    }
    pub fn download_limited(mut self, download_limited: bool) -> Self {
        self.download_limited = Some(download_limited);
        self
    }
    pub fn files_wanted(mut self, files_wanted: Vec<usize>) -> Self {
        self.files_wanted = Some(files_wanted);
        self
    }
    pub fn files_unwanted(mut self, files_unwanted: Vec<usize>) -> Self {
        self.files_unwanted = Some(files_unwanted);
        self
    }

    pub fn ids(mut self, ids: Vec<Id>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn honors_session_limits(mut self, honors_session_limits: bool) -> Self {
        self.honors_session_limits = Some(honors_session_limits);
        self
    }
    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.labels = Some(labels);
        self
    }
    pub fn location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }
    pub fn peer_limit(mut self, peer_limit: u16) -> Self {
        self.peer_limit = Some(peer_limit);
        self
    }
    pub fn priority_high(mut self, priority_high: Vec<usize>) -> Self {
        self.priority_high = Some(priority_high);
        self
    }
    pub fn priority_low(mut self, priority_low: Vec<usize>) -> Self {
        self.priority_low = Some(priority_low);
        self
    }
    pub fn priority_normal(mut self, priority_normal: Vec<usize>) -> Self {
        self.priority_normal = Some(priority_normal);
        self
    }
    pub fn queue_position(mut self, queue_position: usize) -> Self {
        self.queue_position = Some(queue_position);
        self
    }
    pub fn seed_idle_limit(mut self, seed_idle_limit: u16) -> Self {
        self.seed_idle_limit = Some(seed_idle_limit);
        self
    }
    pub fn seed_idle_mode(mut self, seed_idle_mode: IdleMode) -> Self {
        self.seed_idle_mode = Some(seed_idle_mode);
        self
    }
    pub fn seed_ratio_limit(mut self, seed_ratio_limit: f64) -> Self {
        self.seed_ratio_limit = Some(seed_ratio_limit);
        self
    }
    pub fn seed_ratio_mode(mut self, seed_ratio_mode: RatioMode) -> Self {
        self.seed_ratio_mode = Some(seed_ratio_mode);
        self
    }
    pub fn tracker_add(mut self, tracker_add: Vec<String>) -> Self {
        self.tracker_add = Some(tracker_add);
        self
    }
    pub fn tracker_list(mut self, tracker_list: TrackerList) -> Self {
        self.tracker_list = Some(tracker_list);
        self
    }
    pub fn tracker_remove(mut self, tracker_remove: Vec<String>) -> Self {
        self.tracker_remove = Some(tracker_remove);
        self
    }
    pub fn tracker_replace(mut self, tracker_replace: Vec<String>) -> Self {
        self.tracker_replace = Some(tracker_replace);
        self
    }
    pub fn upload_limit(mut self, upload_limit: usize) -> Self {
        self.upload_limit = Some(upload_limit);
        self
    }
    pub fn upload_limited(mut self, upload_limited: bool) -> Self {
        self.upload_limited = Some(upload_limited);
        self
    }
}
