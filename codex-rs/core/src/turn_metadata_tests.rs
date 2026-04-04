use super::*;

use serde_json::Value;
use tempfile::TempDir;

#[tokio::test]
async fn build_turn_metadata_header_only_includes_sandbox() {
    let temp_dir = TempDir::new().expect("temp dir");
    let header = build_turn_metadata_header(temp_dir.path(), Some("none"))
        .await
        .expect("header");
    let parsed: Value = serde_json::from_str(&header).expect("valid json");

    assert_eq!(parsed.get("sandbox").and_then(Value::as_str), Some("none"));
    assert!(parsed.get("workspaces").is_none());
    assert!(parsed.get("session_id").is_none());
}

#[test]
fn turn_metadata_state_uses_platform_sandbox_tag() {
    let temp_dir = TempDir::new().expect("temp dir");
    let cwd = temp_dir.path().to_path_buf();
    let sandbox_policy = SandboxPolicy::new_read_only_policy();

    let state = TurnMetadataState::new(
        "session-a".to_string(),
        "turn-a".to_string(),
        cwd,
        &sandbox_policy,
        WindowsSandboxLevel::Disabled,
    );

    let header = state.current_header_value().expect("header");
    let json: Value = serde_json::from_str(&header).expect("json");
    let sandbox_name = json.get("sandbox").and_then(Value::as_str);

    let expected_sandbox = sandbox_tag(&sandbox_policy, WindowsSandboxLevel::Disabled);
    assert_eq!(sandbox_name, Some(expected_sandbox));
    assert!(json.get("session_id").is_none());
}
