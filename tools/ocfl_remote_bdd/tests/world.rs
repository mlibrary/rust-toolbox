use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct OcflWorld {
    /// Last CLI output (stdout)
    pub last_response_text: Option<String>,
    /// Last object list (for list command)
    pub last_object_list: Vec<String>,
    /// Queue of outputs for multi-step scenarios
    pub output_history: VecDeque<String>,
    /// Path to the OCFL repo root (if needed for file checks)
    pub repo_root: String,
}

