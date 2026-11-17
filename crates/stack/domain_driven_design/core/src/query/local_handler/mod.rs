use crate::layer::ApplicationLayer;
use crate::layer::Layer;
use crate::query::Query;
use std::future::Future;

pub trait LocalQueryHandler<QueryType: Query>: Layer<Layer = ApplicationLayer> {
    fn resolve_query(&self, query: QueryType) -> impl Future<Output = QueryType::Output>;
}
