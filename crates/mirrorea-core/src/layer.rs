use std::collections::BTreeMap;

use serde::Serialize;

use crate::error::{MirroreaCoreError, require_non_empty, require_non_empty_items};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerSignature {
    pub name: String,
    pub requires: Vec<String>,
    pub provides: Vec<String>,
    pub transforms: Vec<String>,
    pub checks: Vec<String>,
    pub emits: Vec<String>,
    pub obligations: Vec<String>,
    pub laws: Vec<String>,
}

impl LayerSignature {
    pub fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("LayerSignature", "name", &self.name)?;
        require_non_empty_items("LayerSignature", "requires", &self.requires)?;
        require_non_empty_items("LayerSignature", "provides", &self.provides)?;
        require_non_empty_items("LayerSignature", "transforms", &self.transforms)?;
        require_non_empty_items("LayerSignature", "checks", &self.checks)?;
        require_non_empty_items("LayerSignature", "emits", &self.emits)?;
        require_non_empty_items("LayerSignature", "obligations", &self.obligations)?;
        require_non_empty_items("LayerSignature", "laws", &self.laws)?;
        Ok(())
    }
}

pub fn layer_signature_lanes() -> Vec<String> {
    [
        "name",
        "requires",
        "provides",
        "transforms",
        "checks",
        "emits",
        "obligations",
        "laws",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}

pub fn insert_layer_signature(
    signatures: &mut BTreeMap<String, LayerSignature>,
    layer: LayerSignature,
) -> Result<(), MirroreaCoreError> {
    layer.validate()?;
    match signatures.get(&layer.name) {
        None => {
            signatures.insert(layer.name.clone(), layer);
            Ok(())
        }
        Some(existing) if existing == &layer => Ok(()),
        Some(existing) => Err(MirroreaCoreError::new(format!(
            "conflicting LayerSignature closeout row for `{}`: existing={existing:?} new={layer:?}",
            layer.name
        ))),
    }
}
