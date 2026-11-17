use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::Query;
use stack_lock_core_name::LockName;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserQuery {
    lock_name: LockName,
}

impl Layer for UserQuery {
    type Layer = ApplicationLayer;
}

impl Query for UserQuery {
    type Output = Result<Vec<String>, Error>;
}

impl UserQuery {
    pub fn lock_name(&self) -> &LockName {
        &self.lock_name
    }
}

impl TryFrom<String> for UserQuery {
    type Error = Error;

    fn try_from(lock_text: String) -> Result<Self, Self::Error> {
        let lock_name = LockName::try_from(lock_text)?;
        Ok(Self { lock_name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_user_query_keeps_its_lock() {
        let user_query = UserQuery::try_from(String::from("rtx3080")).unwrap();

        assert_eq!(user_query.lock_name().text(), "rtx3080");
        assert!(UserQuery::try_from(String::new()).is_err());
    }
}
