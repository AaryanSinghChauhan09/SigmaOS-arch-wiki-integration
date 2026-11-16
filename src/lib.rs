// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-arch-wiki-integration: arch wiki integration component model.
//!
//! Real implementation: integrations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integration {
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
pub enum ArchWikiIntegrationError {
    EmptyIntegrationName,
    DuplicateIntegration(String),
    EmptyEventSubject,
}

impl fmt::Display for ArchWikiIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIntegrationName => write!(f, "integration name must not be empty"),
            Self::DuplicateIntegration(id) => write!(f, "duplicate integration {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ArchWikiIntegrationError {}

pub struct ArchWikiIntegration {
    pub name: String,
    integrations: BTreeMap<String, Integration>,
    events: BTreeMap<String, Event>,
}

impl ArchWikiIntegration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            integrations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_integration(&mut self, item: Integration) -> Result<(), ArchWikiIntegrationError> {
        if item.name.trim().is_empty() {
            return Err(ArchWikiIntegrationError::EmptyIntegrationName);
        }
        let id = item.id.clone();
        if self.integrations.contains_key(&id) {
            return Err(ArchWikiIntegrationError::DuplicateIntegration(id.clone()));
        }
        self.integrations.insert(id, item);
        Ok(())
    }

    pub fn integration_count(&self) -> usize {
        self.integrations.len()
    }

    pub fn get_integration(&self, id: &str) -> Option<&Integration> {
        self.integrations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ArchWikiIntegrationError> {
        if event.subject.trim().is_empty() {
            return Err(ArchWikiIntegrationError::EmptyEventSubject);
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

    fn sample_integration(id: &str, name: &str) -> Integration {
        Integration {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ArchWikiIntegration::new("ArchWikiIntegration");
        assert_eq!(m.name, "ArchWikiIntegration");
        assert_eq!(m.integration_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_integration() {
        let mut m = ArchWikiIntegration::new("ArchWikiIntegration");
        m.add_integration(sample_integration("a1", "Alpha")).unwrap();
        assert_eq!(m.integration_count(), 1);
        assert!(m.get_integration("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ArchWikiIntegration::new("ArchWikiIntegration");
        assert!(m.add_integration(sample_integration("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ArchWikiIntegration::new("ArchWikiIntegration");
        m.add_integration(sample_integration("x", "One")).unwrap();
        assert!(m.add_integration(sample_integration("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ArchWikiIntegration::new("ArchWikiIntegration");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
