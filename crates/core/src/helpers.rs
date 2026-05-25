use crate::repos::{MissingOwnerName, MissingRepositoryName};
use crate::{Capability, CapabilitySet, RepoBuilder};

pub fn capabilities() -> CapabilitySetBuilder {
    CapabilitySetBuilder
}

pub fn repo() -> RepoBuilder<MissingOwnerName, MissingRepositoryName> {
    RepoBuilder {
        owner_name: MissingOwnerName,
        repository_name: MissingRepositoryName,
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapabilitySetBuilder;

impl CapabilitySetBuilder {
    pub fn make(self, capabilities: impl IntoIterator<Item = Capability>) -> CapabilitySet {
        CapabilitySet {
            capabilities: capabilities.into_iter().collect(),
        }
    }
}
