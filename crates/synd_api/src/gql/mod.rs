mod query;
pub use query::Query;

mod mutation;
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
pub use mutation::Mutation;

use crate::{gql::mutation::ResponseCode, principal::Principal, usecase};

pub mod object;
pub mod scalar;

pub type SyndSchema = Schema<Query, Mutation, EmptySubscription>;

pub mod handler {
    use async_graphql::http::GraphiQLSource;
    use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
    use axum::{response::IntoResponse, Extension};
    use synd_o11y::audit_span;
    use tracing::Instrument;

    use crate::principal::Principal;

    use super::SyndSchema;

    pub async fn graphiql() -> impl IntoResponse {
        axum::response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
    }

    pub async fn graphql(
        Extension(schema): Extension<SyndSchema>,
        Extension(principal): Extension<Principal>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        // Inject authentication
        let req = req.into_inner().data(principal);
        schema.execute(req).instrument(audit_span!()).await.into()
    }
}

#[must_use]
pub fn schema_builder() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    let schema = Schema::build(Query, Mutation, EmptySubscription);

    if cfg!(not(feature = "introspection")) {
        schema
            .disable_introspection()
            .limit_depth(10)
            .limit_complexity(60)
    } else {
        schema.limit_depth(20).limit_complexity(300)
    }
    // disabled
    // schema.extension(Tracing)
}

impl<'a> usecase::Context for &async_graphql::Context<'a> {
    fn principal(&self) -> Principal {
        self.data_unchecked::<Principal>().clone()
    }
}

impl<E> async_graphql::ErrorExtensions for usecase::Error<E>
where
    E: std::fmt::Display + Send + Sync + 'static,
{
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{self}")).extend_with(|_, ext| match self {
            usecase::Error::Usecase(_) => unreachable!(),
            usecase::Error::Unauthorized(_) => ext.set("code", ResponseCode::Unauthorized),
            usecase::Error::Repository(_) => ext.set("code", ResponseCode::InternalError),
        })
    }
}

impl async_graphql::ErrorExtensions for usecase::FetchEntriesError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{self}"))
            .extend_with(|_, ext| ext.set("code", ResponseCode::InternalError))
    }
}

impl async_graphql::ErrorExtensions for usecase::FetchSubscribedFeedsError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{self}"))
            .extend_with(|_, ext| ext.set("code", ResponseCode::InternalError))
    }
}

macro_rules! run_usecase {
    ($usecase:ty, $cx:expr, $input:expr,$err_handle:expr) => {{
        let runtime = $cx.data_unchecked::<crate::usecase::Runtime>();
        let err_handle = $err_handle;

        match runtime.run::<$usecase, _, _>($cx, $input).await {
            Ok(output) => Ok(output.into()),
            Err($crate::usecase::Error::Usecase(uc_err)) => err_handle(uc_err),
            Err(err) => Err(async_graphql::ErrorExtensions::extend(&err)),
        }
    }};
}

pub(super) use run_usecase;
