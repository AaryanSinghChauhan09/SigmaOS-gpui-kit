// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-gpui-kit: gpui kit component model.
//!
//! Real implementation: kits, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kit {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuiKitError {
    EmptyKitName,
    DuplicateKit(String),
    EmptyEventSubject,
}

impl fmt::Display for GpuiKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKitName => write!(f, "kit name must not be empty"),
            Self::DuplicateKit(id) => write!(f, "duplicate kit {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GpuiKitError {}

pub struct GpuiKit {
    pub name: String,
    kits: BTreeMap<String, Kit>,
    events: BTreeMap<String, Event>,
}

impl GpuiKit {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kits: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_kit(&mut self, item: Kit) -> Result<(), GpuiKitError> {
        if item.name.trim().is_empty() {
            return Err(GpuiKitError::EmptyKitName);
        }
        let id = item.id.clone();
        if self.kits.contains_key(&id) {
            return Err(GpuiKitError::DuplicateKit(id.clone()));
        }
        self.kits.insert(id, item);
        Ok(())
    }

    pub fn kit_count(&self) -> usize {
        self.kits.len()
    }

    pub fn get_kit(&self, id: &str) -> Option<&Kit> {
        self.kits.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GpuiKitError> {
        if event.subject.trim().is_empty() {
            return Err(GpuiKitError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kit(id: &str, name: &str) -> Kit {
        Kit {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GpuiKit::new("GpuiKit");
        assert_eq!(m.name, "GpuiKit");
        assert_eq!(m.kit_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_kit() {
        let mut m = GpuiKit::new("GpuiKit");
        m.add_kit(sample_kit("a1", "Alpha")).unwrap();
        assert_eq!(m.kit_count(), 1);
        assert!(m.get_kit("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GpuiKit::new("GpuiKit");
        assert!(m.add_kit(sample_kit("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GpuiKit::new("GpuiKit");
        m.add_kit(sample_kit("x", "One")).unwrap();
        assert!(m.add_kit(sample_kit("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GpuiKit::new("GpuiKit");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
