// workflow_logger.rs
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, Duration};
use uuid::Uuid;
use chrono::Local;

/// A structure to track and log timing information for workflow stages.
pub struct WorkflowTimings {
    workflow_id: String,
    start_time: SystemTime,
    stage_start_time: SystemTime, // To measure current stage
    stages_data: Vec<(String, Duration)>, // (stage_name, stage_duration)
    log_file_path: String,
}

impl WorkflowTimings {
    /// Creates a new WorkflowTimings instance with a unique ID and logs the start event.
    /// 
    /// # Arguments
    /// * `id_prefix` - A prefix for the workflow ID to categorize the type of workflow.
    /// * `log_file` - The file path where logs will be written.
    /// 
    /// # Returns
    /// A new instance of WorkflowTimings.
    pub fn new(id_prefix: &str, log_file: &str) -> Self {
        let id = format!("{}-{}", id_prefix, Uuid::new_v4().to_string());
        let now = SystemTime::now();
        
        // Ensure the logs directory exists
        if let Some(parent_dir) = Path::new(log_file).parent() {
            if !parent_dir.exists() {
                if let Err(e) = fs::create_dir_all(parent_dir) {
                    eprintln!("[ERROR] Failed to create logs directory {}: {}", parent_dir.display(), e);
                } else {
                    println!("[INFO] Created logs directory: {}", parent_dir.display());
                }
            }
        }
        
        let instance = WorkflowTimings {
            workflow_id: id.clone(),
            start_time: now,
            stage_start_time: now, // Initialize stage start time
            stages_data: Vec::new(),
            log_file_path: log_file.to_string(),
        };
        // Log workflow start immediately
        instance.log_entry("START", "Voice input processing initiated.");
        instance
    }

    /// Resets the start time for the next stage.
    pub fn start_new_stage(&mut self) {
        self.stage_start_time = SystemTime::now();
    }
    
    /// Records the duration of the current stage and logs it.
    /// 
    /// # Arguments
    /// * `stage_name` - The name of the stage that just completed.
    pub fn record_stage(&mut self, stage_name: String) {
        let duration = SystemTime::now().duration_since(self.stage_start_time)
            .unwrap_or_default();
        self.stages_data.push((stage_name.clone(), duration));
        self.log_entry("STAGE", &format!("Stage: {} | DURATION: {}ms", stage_name, duration.as_millis()));
        self.start_new_stage(); // Prepare for the next stage
    }

    /// Finalizes the workflow timing, calculates overall statistics, and logs the end event.
    /// 
    /// # Arguments
    /// * `overall_status_message` - A message describing the final status of the workflow.
    pub fn finalize_and_log(&self, overall_status_message: &str) {
        let total_duration = SystemTime::now().duration_since(self.start_time).unwrap_or_default();
        let num_stages = self.stages_data.len();
        let avg_stage_time_ms = if num_stages > 0 {
            self.stages_data.iter().map(|s| s.1.as_millis()).sum::<u128>() / num_stages as u128
        } else {
            0
        };

        let log_message = format!(
            "END: {} | TOTAL_E2E: {}ms | STAGES_COUNT: {} | AVG_STAGE_TIME: {}ms",
            overall_status_message,
            total_duration.as_millis(),
            num_stages,
            avg_stage_time_ms
        );
        self.log_entry("END", &log_message);
    }

    /// Logs an entry to the specified log file with a timestamp and event details.
    /// 
    /// # Arguments
    /// * `event_type` - The type of event being logged (e.g., START, STAGE, END).
    /// * `message` - The message or details of the event.
    fn log_entry(&self, event_type: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let log_line = format!("[{}] ID: {} | {} | {}\n", timestamp, self.workflow_id, event_type, message);
        
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&self.log_file_path) {
            if let Err(e) = file.write_all(log_line.as_bytes()) {
                eprintln!("[ERROR] Failed to write to workflow log: {}", e);
            }
        } else {
            eprintln!("[ERROR] Failed to open workflow log file: {}", self.log_file_path);
        }
    }
}
