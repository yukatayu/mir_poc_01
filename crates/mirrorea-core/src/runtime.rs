use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{MirroreaCoreError, require_non_empty};

const PARTICIPANT_PLACE_KIND: &str = "ParticipantPlace";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
        if self.active && self.left_at_epoch.is_some() {
            return Err(MirroreaCoreError::new(format!(
                "MemberRecord `{}` is active but has left_at_epoch set",
                self.principal
            )));
        }
        if !self.active && self.left_at_epoch.is_none() {
            return Err(MirroreaCoreError::new(format!(
                "MemberRecord `{}` is inactive but has no left_at_epoch",
                self.principal
            )));
        }
        if let Some(left_at_epoch) = self.left_at_epoch {
            if left_at_epoch < self.joined_at_epoch {
                return Err(MirroreaCoreError::new(format!(
                    "MemberRecord `{}` has left_at_epoch {} before joined_at_epoch {}",
                    self.principal, left_at_epoch, self.joined_at_epoch
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemberSnapshot {
    pub place: String,
    pub active: bool,
    pub incarnation: u64,
    pub joined_at_epoch: u64,
    pub left_at_epoch: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MembershipSnapshot {
    pub membership_epoch: u64,
    pub members: BTreeMap<String, MemberSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
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

    pub fn restore(snapshot: &MembershipSnapshot) -> Result<Self, MirroreaCoreError> {
        let mut members = BTreeMap::new();
        for (principal, member) in &snapshot.members {
            require_non_empty("MembershipSnapshot", "principal", principal)?;
            require_non_empty("MembershipSnapshot", "place", &member.place)?;
            let record = MemberRecord {
                principal: principal.clone(),
                participant_place: member.place.clone(),
                active: member.active,
                incarnation: member.incarnation,
                joined_at_epoch: member.joined_at_epoch,
                left_at_epoch: member.left_at_epoch,
            };
            record.validate()?;
            if record.joined_at_epoch > snapshot.membership_epoch {
                return Err(MirroreaCoreError::new(format!(
                    "MembershipSnapshot principal `{principal}` joined_at_epoch {} exceeds membership frontier {}",
                    record.joined_at_epoch, snapshot.membership_epoch
                )));
            }
            if let Some(left_at_epoch) = record.left_at_epoch {
                if left_at_epoch > snapshot.membership_epoch {
                    return Err(MirroreaCoreError::new(format!(
                        "MembershipSnapshot principal `{principal}` left_at_epoch {} exceeds membership frontier {}",
                        left_at_epoch, snapshot.membership_epoch
                    )));
                }
            }
            members.insert(principal.clone(), record);
        }

        Ok(Self {
            membership_epoch: snapshot.membership_epoch,
            members,
        })
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlaceCatalogSnapshot {
    pub places: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
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

    pub fn restore(snapshot: &PlaceCatalogSnapshot) -> Result<Self, MirroreaCoreError> {
        let mut places = BTreeMap::new();
        for (place_id, kind) in &snapshot.places {
            require_non_empty("PlaceCatalogSnapshot", "place_id", place_id)?;
            require_non_empty("PlaceCatalogSnapshot", "kind", kind)?;
            places.insert(place_id.clone(), kind.clone());
        }
        Ok(Self { places })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogicalPlaceRuntimeSnapshot {
    pub place_catalog: PlaceCatalogSnapshot,
    pub membership: MembershipSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LogicalPlaceRuntimeShell {
    place_catalog: PlaceCatalog,
    membership: MembershipRegistry,
}

impl LogicalPlaceRuntimeShell {
    pub fn register_place(&mut self, place_id: &str, kind: &str) -> Result<(), MirroreaCoreError> {
        self.place_catalog.register(place_id, kind)
    }

    pub fn register_participant_place(
        &mut self,
        principal: &str,
    ) -> Result<String, MirroreaCoreError> {
        let participant_place = self.participant_place_for(principal)?;
        self.register_place(&participant_place, PARTICIPANT_PLACE_KIND)?;
        Ok(participant_place)
    }

    pub fn add_initial_member(
        &mut self,
        principal: &str,
        participant_place: &str,
    ) -> Result<(), MirroreaCoreError> {
        self.ensure_registered_participant_place(participant_place)?;
        self.membership.add_initial(principal, participant_place)
    }

    pub fn add_member(
        &mut self,
        principal: &str,
        participant_place: &str,
    ) -> Result<MemberRecord, MirroreaCoreError> {
        self.ensure_registered_participant_place(participant_place)?;
        self.membership.add_member(principal, participant_place)
    }

    pub fn mark_inactive_member(
        &mut self,
        principal: &str,
    ) -> Result<MemberRecord, MirroreaCoreError> {
        self.membership.mark_inactive(principal)
    }

    pub fn add_initial_participant(&mut self, principal: &str) -> Result<(), MirroreaCoreError> {
        self.with_registered_participant_place(
            principal,
            |membership, principal, participant_place| {
                membership.add_initial(principal, participant_place)
            },
        )
    }

    pub fn add_participant(&mut self, principal: &str) -> Result<MemberRecord, MirroreaCoreError> {
        self.with_registered_participant_place(
            principal,
            |membership, principal, participant_place| {
                membership.add_member(principal, participant_place)
            },
        )
    }

    pub fn leave_participant(
        &mut self,
        principal: &str,
    ) -> Result<MemberRecord, MirroreaCoreError> {
        self.mark_inactive_member(principal)
    }

    pub fn snapshot(&self) -> LogicalPlaceRuntimeSnapshot {
        LogicalPlaceRuntimeSnapshot {
            place_catalog: self.place_catalog.snapshot(),
            membership: self.membership.snapshot(),
        }
    }

    pub fn restore(snapshot: &LogicalPlaceRuntimeSnapshot) -> Result<Self, MirroreaCoreError> {
        let shell = Self {
            place_catalog: PlaceCatalog::restore(&snapshot.place_catalog)?,
            membership: MembershipRegistry::restore(&snapshot.membership)?,
        };

        for (principal, member) in &snapshot.membership.members {
            shell.ensure_registered_participant_place(&member.place)?;
            let expected_place = shell.participant_place_for(principal)?;
            if member.place != expected_place {
                return Err(MirroreaCoreError::new(format!(
                    "LogicalPlaceRuntimeShell restore requires principal `{principal}` to use participant place `{expected_place}`, found `{}`",
                    member.place
                )));
            }
        }

        Ok(shell)
    }

    fn ensure_registered_participant_place(
        &self,
        participant_place: &str,
    ) -> Result<(), MirroreaCoreError> {
        let kind = self.place_catalog.kind_of(participant_place).ok_or_else(|| {
            MirroreaCoreError::new(format!(
                "LogicalPlaceRuntimeShell requires registered place `{participant_place}` before membership mutation"
            ))
        })?;
        if kind != PARTICIPANT_PLACE_KIND {
            return Err(MirroreaCoreError::new(format!(
                "LogicalPlaceRuntimeShell requires `{participant_place}` to have kind `{PARTICIPANT_PLACE_KIND}` before membership mutation, found `{kind}`"
            )));
        }
        Ok(())
    }

    fn participant_place_for(&self, principal: &str) -> Result<String, MirroreaCoreError> {
        require_non_empty("LogicalPlaceRuntimeShell", "principal", principal)?;
        Ok(format!("ParticipantPlace[{principal}]"))
    }

    fn with_registered_participant_place<R>(
        &mut self,
        principal: &str,
        mutate: impl FnOnce(&mut MembershipRegistry, &str, &str) -> Result<R, MirroreaCoreError>,
    ) -> Result<R, MirroreaCoreError> {
        let participant_place = self.participant_place_for(principal)?;
        let existed = self.place_catalog.kind_of(&participant_place).is_some();
        self.place_catalog
            .register(&participant_place, PARTICIPANT_PLACE_KIND)?;
        match mutate(&mut self.membership, principal, &participant_place) {
            Ok(result) => Ok(result),
            Err(err) => {
                if !existed {
                    self.place_catalog.places.remove(&participant_place);
                }
                Err(err)
            }
        }
    }
}
