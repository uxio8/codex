use std::path::Path;
use std::path::PathBuf;

use serde::Serialize;

use crate::sandbox_tags::sandbox_tag;
use codex_protocol::config_types::WindowsSandboxLevel;
use codex_protocol::protocol::SandboxPolicy;

#[derive(Clone, Debug, Serialize, Default)]
pub(crate) struct TurnMetadataBag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    turn_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sandbox: Option<String>,
}

impl TurnMetadataBag {
    fn to_header_value(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }
}

fn build_turn_metadata_bag(turn_id: Option<String>, sandbox: Option<String>) -> TurnMetadataBag {
    TurnMetadataBag { turn_id, sandbox }
}

pub async fn build_turn_metadata_header(cwd: &Path, sandbox: Option<&str>) -> Option<String> {
    let _ = cwd;
    sandbox.map(ToString::to_string).and_then(|sandbox| {
        build_turn_metadata_bag(/*turn_id*/ None, Some(sandbox)).to_header_value()
    })
}

#[derive(Clone, Debug)]
pub(crate) struct TurnMetadataState {
    base_header: String,
}

impl TurnMetadataState {
    pub(crate) fn new(
        session_id: String,
        turn_id: String,
        cwd: PathBuf,
        sandbox_policy: &SandboxPolicy,
        windows_sandbox_level: WindowsSandboxLevel,
    ) -> Self {
        let _ = session_id;
        let _ = cwd;
        let sandbox = Some(sandbox_tag(sandbox_policy, windows_sandbox_level).to_string());
        let base_header = build_turn_metadata_bag(Some(turn_id), sandbox)
            .to_header_value()
            .unwrap_or_else(|| "{}".to_string());

        Self { base_header }
    }

    pub(crate) fn current_header_value(&self) -> Option<String> {
        Some(self.base_header.clone())
    }

    pub(crate) fn current_meta_value(&self) -> Option<serde_json::Value> {
        self.current_header_value()
            .and_then(|header| serde_json::from_str(&header).ok())
    }

    pub(crate) fn spawn_git_enrichment_task(&self) {}

    pub(crate) fn cancel_git_enrichment_task(&self) {}
}

#[cfg(test)]
#[path = "turn_metadata_tests.rs"]
mod tests;
