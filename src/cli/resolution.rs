use crate::cli::{Base, Extra};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ResolveError {
    #[error("cannot both enable and disable highlighters")]
    ConflictEnableDisable,
}

#[derive(Debug)]
pub(crate) struct BaseSet(HashSet<Base>);

impl BaseSet {
    pub(crate) fn resolve(enabled: &[Base], disabled: &[Base]) -> Result<Self, ResolveError> {
        let set = match (enabled.is_empty(), disabled.is_empty()) {
            (true, true) => Base::ALL.iter().copied().collect(),
            (false, true) => enabled.iter().copied().collect(),
            (true, false) => Base::ALL.iter().copied().filter(|g| !disabled.contains(g)).collect(),
            (false, false) => return Err(ResolveError::ConflictEnableDisable),
        };
        Ok(Self(set))
    }

    pub(crate) fn contains(&self, base: Base) -> bool {
        self.0.contains(&base)
    }
}

pub(crate) fn resolve_extras(extras: &[Extra]) -> HashSet<Extra> {
    extras.iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_inputs_enable_all_base_groups() {
        let set = BaseSet::resolve(&[], &[]).unwrap();
        for base in Base::ALL {
            assert!(set.contains(*base), "expected {base:?} to be enabled");
        }
    }

    #[test]
    fn enabled_list_is_exclusive() {
        let set = BaseSet::resolve(&[Base::Json, Base::Numbers], &[]).unwrap();
        assert!(set.contains(Base::Json));
        assert!(set.contains(Base::Numbers));
        assert!(!set.contains(Base::Urls));
        assert!(!set.contains(Base::Quotes));
    }

    #[test]
    fn disabled_list_subtracts_from_all() {
        let set = BaseSet::resolve(&[], &[Base::Json, Base::Numbers]).unwrap();
        assert!(!set.contains(Base::Json));
        assert!(!set.contains(Base::Numbers));
        assert!(set.contains(Base::Urls));
        assert!(set.contains(Base::Quotes));
    }

    #[test]
    fn both_lists_nonempty_is_an_error() {
        let err = BaseSet::resolve(&[Base::Json], &[Base::Numbers]).unwrap_err();
        assert!(matches!(err, ResolveError::ConflictEnableDisable));
    }
}
