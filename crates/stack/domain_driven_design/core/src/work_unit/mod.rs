use std::future::Future;

pub trait WorkUnit {
    type Error;

    fn try_commit_changes(self) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
