//! A read-only frame shared by future Animation playback and export.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use solidcraft_doc::joints;
use solidcraft_geom::Vec3;

use crate::params::bad;
use crate::{Result, Session};

use super::{pose_json, study_index, view_index};

pub(super) fn sample(s: &mut Session, p: &Value) -> Result<Value> {
    let cmd = "animation.frame";
    let progress = p
        .get("progress")
        .and_then(Value::as_f64)
        .filter(|v| v.is_finite() && (0.0..=1.0).contains(v))
        .ok_or_else(|| bad(cmd, "`progress` must be 0…1"))?;
    let study = p.get("study").filter(|v| !v.is_null()).map(|_| study_index(s, p, cmd)).transpose()?;
    let view = p.get("view").filter(|v| !v.is_null()).map(|_| view_index(s, p, cmd)).transpose()?;
    if study.is_none() && view.is_none() {
        return Err(bad(cmd, "`study` or `view` is required"));
    }

    let mut trial = s.scratch();
    trial.pending_moves.clear();
    let motion = match study.and_then(|i| s.doc.assembly.studies.get(i)) {
        Some(m) => Some(super::play(&mut trial, &json!({"study": m.id, "step": progress * f64::from(m.steps), "quiet": true}))?),
        None => None,
    };
    let mut moves = BTreeMap::<u64, Vec3>::new();
    if let Some(view) = view.and_then(|i| s.doc.assembly.exploded.get(i)) {
        for (id, offset) in &view.moves {
            if !offset.x.is_finite() || !offset.y.is_finite() || !offset.z.is_finite() || offset.len() >= 1e7 {
                return Err(bad(cmd, "exploded view has an invalid translation"));
            }
            *moves.entry(*id).or_insert(Vec3::ZERO) += *offset * progress;
        }
    }
    let positions: Vec<Value> = trial
        .doc
        .occurrences
        .iter()
        .map(|o| {
            let mut pose = joints::occurrence_world(&trial.doc, o.id);
            for id in trial.doc.first_path(o.parent).into_iter().chain([o.id]) {
                if let Some(Vec3 { x, y, z }) = moves.get(&id) {
                    pose[3][0] += *x;
                    pose[3][1] += *y;
                    pose[3][2] += *z;
                }
            }
            json!({"id": o.id, "occurrence": o.name, "pose": pose_json(&pose)})
        })
        .collect();
    Ok(json!({"progress": progress, "positions": positions, "motion": motion}))
}
