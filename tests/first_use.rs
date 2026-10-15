use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::{Value, json};
use tempfile::TempDir;

fn run(home: &Path, state: &Path, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_skillroster"))
        .arg("--home")
        .arg(home)
        .arg("--state-dir")
        .arg(state)
        .arg("--json")
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["ok"], true);
    value
}

#[test]
fn informational_only_reports_can_end_without_losing_finding_access_or_reusing_old_summaries() {
    for has_skill in [false, true] {
        let temp = TempDir::new().unwrap();
        let home = temp.path().join("home");
        let state = temp.path().join("state");
        fs::create_dir_all(&home).unwrap();
        if has_skill {
            let skill = home.join(".codex/skills/review");
            fs::create_dir_all(&skill).unwrap();
            fs::write(skill.join("SKILL.md"), "---\nname: review\ndescription: Review source changes\nsource: fixture/review\nversion: 1\n---\n").unwrap();
        }
        run(&home, &state, &["scan"]);
        let report = run(&home, &state, &["report"]);
        assert_eq!(
            report["result"]["overview"]["state"],
            if has_skill {
                "no_review_items"
            } else {
                "inventory_empty"
            }
        );
        assert_eq!(report["result"]["findings"], json!([]));
        let all = run(&home, &state, &["report", "--full"]);
        let count = all["result"]["findings"].as_array().unwrap().len();
        assert!(count > 0);
        assert_eq!(report["result"]["finding_count"], count);
        let list = run(&home, &state, &["report", "--findings"]);
        assert_eq!(list["result"]["items"].as_array().unwrap().len(), count);
        let finding_id = list["result"]["items"][0]["id"].as_str().unwrap();
        assert_eq!(
            run(&home, &state, &["report", "--finding", finding_id])["result"]["id"],
            finding_id
        );
        let db = rusqlite::Connection::open(state.join("skillroster.db")).unwrap();
        let mut legacy = all["result"].clone();
        legacy.as_object_mut().unwrap().remove("overview");
        db.execute(
            "UPDATE reports SET summary_json = ?1 WHERE id = ?2",
            rusqlite::params![
                legacy.to_string(),
                report["result"]["report_id"].as_str().unwrap()
            ],
        )
        .unwrap();
        let refreshed = run(&home, &state, &["report"]);
        assert_ne!(
            refreshed["result"]["report_id"],
            report["result"]["report_id"]
        );
        assert_eq!(
            refreshed["result"]["snapshot_id"],
            report["result"]["snapshot_id"]
        );
        assert_eq!(
            refreshed["result"]["overview"],
            report["result"]["overview"]
        );
        assert_eq!(
            run(&home, &state, &["report"])["result"]["report_id"],
            refreshed["result"]["report_id"]
        );
        let plans: i64 = db
            .query_row("SELECT COUNT(*) FROM plans", [], |row| row.get(0))
            .unwrap();
        assert_eq!(plans, 0);
    }
}

#[test]
fn bounded_inventory_remains_a_review_item_even_when_no_skills_are_found() {
    let temp = TempDir::new().unwrap();
    let home = temp.path().join("home");
    let state = temp.path().join("state");
    fs::create_dir_all(home.join(".codex/skills/a/b/c/d/e/f/g")).unwrap();
    run(&home, &state, &["scan"]);
    let report = run(&home, &state, &["report"]);
    assert_eq!(report["result"]["skill_count"], 0);
    assert_eq!(report["result"]["overview"]["state"], "review_available");
    assert!(
        report["result"]["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|finding| finding["kind"] == "configured_roots_bounded")
    );
}

#[test]
fn integration_inspection_never_creates_a_plan_and_tracks_setup_apply_and_undo() {
    let temp = TempDir::new().unwrap();
    let home = temp.path().join("home");
    let state = temp.path().join("state");
    fs::create_dir_all(home.join(".codex/skills")).unwrap();
    run(&home, &state, &["scan"]);
    run(&home, &state, &["report"]);
    let home_view = run(&home, &state, &[]);
    assert_eq!(home_view["result"]["state"], "ready");
    assert_eq!(
        home_view["result"]["agent_integration"]["state"],
        "setup_available"
    );
    assert_eq!(home_view["result"]["pending_plan_count"], 0);
    let check = run(&home, &state, &["setup", "--check"]);
    assert_eq!(
        check["result"]["targets"],
        home_view["result"]["agent_integration"]["targets"]
    );
    assert_eq!(check["result"]["plan_id"], Value::Null);
    let db = rusqlite::Connection::open(state.join("skillroster.db")).unwrap();
    assert_eq!(
        db.query_row("SELECT COUNT(*) FROM plans", [], |row| row.get::<_, i64>(0))
            .unwrap(),
        0
    );
    assert!(!home.join(".codex/skills/skillroster").exists());
    let setup = run(&home, &state, &["setup"]);
    let applied = run(
        &home,
        &state,
        &["apply", setup["result"]["plan_id"].as_str().unwrap()],
    );
    assert_eq!(applied["result"]["verification"], "passed");
    run(&home, &state, &["scan"]);
    assert_eq!(
        run(&home, &state, &[])["result"]["agent_integration"]["state"],
        "up_to_date"
    );
    run(
        &home,
        &state,
        &["undo", applied["result"]["receipt_id"].as_str().unwrap()],
    );
    run(&home, &state, &["scan"]);
    assert_eq!(
        run(&home, &state, &["setup", "--check"])["result"]["state"],
        "setup_available"
    );
    assert!(!home.join(".codex/skills/skillroster").exists());
}
