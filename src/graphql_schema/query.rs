use juniper::{
    meta::{MetaType, Field}, Arguments, EmptyMutation, EmptySubscription, ExecutionResult, Executor,
    GraphQLType,GraphQLValueAsync, GraphQLValue, Registry, RootNode, ScalarValue,
};
use sea_schema::sqlite::{Schema};
use super::single_query::{SingleQuery, SingleQueryInfo};

pub async fn get_graphql_schema(schema: &Schema) -> RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>> {
    let info = QueryTypeInfo {
        name: "Query".into(),
        schema: schema.clone(),
    };

    let schema: RootNode<'_, _, _, _> = RootNode::new_with_info(
        Query {},
        EmptyMutation::new(),
        EmptySubscription::new(),
        info,
        (),
        (),
    );

    schema
}

#[derive(Debug)]
pub struct Query;

impl<S> GraphQLType<S> for Query
where
    S: ScalarValue
{
    fn name(info: &QueryTypeInfo) -> Option<&str> {
        Some(info.name.as_str())
    }

    fn meta<'r>(info: &QueryTypeInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let single_queries: Vec<Field<S>> = info
            .schema
            .tables
            .iter()
            .map(|table| {
                let name = format!("single_{}", table.name.as_str());
                let query = SingleQuery {
                    info: SingleQueryInfo {
                        name,
                        meta: table.clone()
                    }
                };

                println!("A: {:?}", query);

                registry
                    .field::<SingleQuery>(query.info.name.as_str(), &query.info)
                    .argument(
                        registry.arg::<String>("id", &())
                    )
            })
            .collect();

        let batch_queries: Vec<Field<S>> = info
            .schema
            .tables
            .iter()
            .map(|table| {
                let name = format!("batch_{}", table.name.as_str());
                let query = SingleQuery {
                    info: SingleQueryInfo {
                        name,
                        meta: table.clone()
                    }
                };

                println!("A: {:?}", query);

                registry
                    .field::<Vec<SingleQuery>>(query.info.name.as_str(), &query.info)
            })
            .collect();

        let fields = [single_queries, batch_queries].concat();

        registry
            .build_object_type::<Query>(info, &fields)
            .into_meta()
    }
}

impl<S> GraphQLValue<S> for Query
where
    S: ScalarValue,
{
    type Context = ();
    type TypeInfo = QueryTypeInfo;

    fn type_name<'i>(&self, info: &'i QueryTypeInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }
}

impl<S> GraphQLValueAsync<S> for Query
where
    S: ScalarValue + Send + Sync
{

}

pub struct QueryTypeInfo {
    pub name: String,
    pub schema: Schema
}

pub enum QueryTypesEnum {
    Single(SingleQuery)
}