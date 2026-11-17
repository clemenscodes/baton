use crate::layer::ApplicationLayer;
use crate::layer::Layer;
use crate::query::Query;

pub trait SynchronousQueryHandler<QueryType: Query>: Layer<Layer = ApplicationLayer> {
    fn resolve_query(&self, query: QueryType) -> QueryType::Output;
}
