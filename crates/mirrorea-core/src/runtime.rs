use std::collections::BTreeMap;

use serde::Serialize;

use crate::error::{MirroreaCoreError, require_non_empty};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MemberRecord {
    pub principal: String,
    pub participant_place: String,
    pub active: bool,
    pub incarnation: u64,
    pub joined_at_epoch: u64,
    pub left_at_epoch: Option<u64>,
}

impl MemberRecord {
    pub fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("MemberRecord", "principal", &self.principal)?;
        require_non_empty("MemberRecord", "participant_place", &self.participant_place)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MemberSnapshot {
    pub place: String,
    pub active: bool,
    pub incarnation: u64,
    pub joined_at_epoch: u64,
    pub left_at_epoch: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MembershipSnapshot {
    pub membership_epoch: u64,
    pub members: BTreeMap<String, MemberSnapshot>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct MembershipRegistry {
    membership_epoch: u64,
    members: BTreeMap<String, MemberRecord>,
}

impl MembershipRegistry {
    pub fn membership_epoch(&self) -> u64 {
        self.membership_epoch
    }

    pub fn add_initial(
        &mut self,
        principal: &str,
        participant_place: &str,
    ) -> Result<(), MirroreaCoreError> {
        if self.membership_epoch != 0 {
            return Err(MirroreaCoreError::new(
                "MembershipRegistry::add_initial is bootstrap-only once membership_epoch has advanced",
            ));
        }
        let record = self.new_member_record(principal, participant_place, self.membership_epoch)?;
        self.members.insert(principal.to_string(), record);
        Ok(())
    }

    pub fn add_member(
        &mut self,
        principal: &str,
        participant_place: &str,
    ) -> Result<MemberRecord, MirroreaCoreError> {
        let joined_at_epoch = self.membership_epoch + 1;
        let record = self.new_member_record(principal, participant_place, joined_at_epoch)?;
        self.membership_epoch = joined_at_epoch;
        self.members.insert(principal.to_string(), record.clone());
        Ok(record)
    }

    pub fn mark_inactive(&mut self, principal: &str) -> Result<MemberRecord, MirroreaCoreError> {
        let record = self.members.get_mut(principal).ok_or_else(|| {
            MirroreaCoreError::new(format!(
                "MembershipRegistry principal `{principal}` does not exist"
            ))
        })?;
        if !record.active {
            return Err(MirroreaCoreError::new(format!(
                "MembershipRegistry principal `{principal}` is already inactive"
            )));
        }
        self.membership_epoch += 1;
        record.active = false;
        record.incarnation += 1;
        record.left_at_epoch = Some(self.membership_epoch);
        Ok(record.clone())
    }

    pub fn active(&self, principal: &str) -> bool {
        self.members
            .get(principal)
            .map(|record| record.active)
            .unwrap_or(false)
    }

    pub fn active_members(&self) -> Vec<String> {
        self.members
            .iter()
            .filter_map(|(principal, record)| record.active.then_some(principal.clone()))
            .collect()
    }

    pub fn inactive_members(&self) -> Vec<String> {
        self.members
            .iter()
            .filter_map(|(principal, record)| (!record.active).then_some(principal.clone()))
            .collect()
    }

    pub fn snapshot(&self) -> MembershipSnapshot {
        MembershipSnapshot {
            membership_epoch: self.membership_epoch,
            members: self
                .members
                .iter()
                .map(|(principal, record)| {
                    (
                        principal.clone(),
                        MemberSnapshot {
                            place: record.participant_place.clone(),
                            active: record.active,
                            incarnation: record.incarnation,
                            joined_at_epoch: record.joined_at_epoch,
                            left_at_epoch: record.left_at_epoch,
                        },
                    )
                })
                .collect(),
        }
    }

    fn new_member_record(
        &self,
        principal: &str,
        participant_place: &str,
        joined_at_epoch: u64,
    ) -> Result<MemberRecord, MirroreaCoreError> {
        require_non_empty("MembershipRegistry", "principal", principal)?;
        require_non_empty("MembershipRegistry", "participant_place", participant_place)?;
        if self.members.contains_key(principal) {
            return Err(MirroreaCoreError::new(format!(
                "MembershipRegistry principal `{principal}` already exists; rejoin semantics remain unresolved"
            )));
        }
        let record = MemberRecord {
            principal: principal.to_string(),
            participant_place: participant_place.to_string(),
            active: true,
            incarnation: 0,
            joined_at_epoch,
            left_at_epoch: None,
        };
        record.validate()?;
        Ok(record)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct PlaceCatalogSnapshot {
    pub places: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct PlaceCatalog {
    places: BTreeMap<String, String>,
}

impl PlaceCatalog {
    pub fn register(&mut self, place_id: &str, kind: &str) -> Result<(), MirroreaCoreError> {
        require_non_empty("PlaceCatalog", "place_id", place_id)?;
        require_non_empty("PlaceCatalog", "kind", kind)?;
        match self.places.get(place_id) {
            Some(existing_kind) if existing_kind == kind => Ok(()),
            Some(existing_kind) => Err(MirroreaCoreError::new(format!(
                "PlaceCatalog place `{place_id}` already exists with kind `{existing_kind}`, cannot replace with `{kind}`"
            ))),
            None => {
                self.places.insert(place_id.to_string(), kind.to_string());
                Ok(())
            }
        }
    }

    pub fn kind_of(&self, place_id: &str) -> Option<&str> {
        self.places.get(place_id).map(String::as_str)
    }

    pub fn place_ids(&self) -> Vec<String> {
        self.places.keys().cloned().collect()
    }

    pub fn snapshot(&self) -> PlaceCatalogSnapshot {
        PlaceCatalogSnapshot {
            places: self.places.clone(),
        }
    }
}
