use juniper::{
    meta::MetaType, Arguments, EmptyMutation, EmptySubscription, ExecutionResult, Executor,
    GraphQLType,GraphQLValueAsync, GraphQLValue, Registry, RootNode, ScalarValue
};
use sea_schema::sqlite::{Schema};

pub async fn get_graphql_schema(schema: &Schema) -> RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>> {
    let fields = schema
        .tables
        .iter()
        .map(|table| {
            let fields = table
                .columns
                .iter()
                .map(|column| {
                    QueryTypeInfo {
                        name: upper(&column.name),
                        fields: Vec::new()
                    }
                })
                .collect();

            QueryTypeInfo {
                name: format!("All{}", upper(&table.name)),
                fields
            }
        })
        .collect();

    let info = QueryTypeInfo {
        name: "Query".into(),
        fields,
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
    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(info.name.as_str())
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = info
            .fields
            .iter()
            .map(|field| {
                println!("A: {:?}", field);
                registry
                    .field::<Vec<QueryTypeInfo>>(field.name.as_str(), field)
                    .argument(
                        registry.arg::<String>("id", &())
                    )
            })
            .collect::<Vec<_>>();

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

    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }
}

impl<S> GraphQLValueAsync<S> for Query
where
    S: ScalarValue + Send + Sync
{

}

#[derive(Debug)]
pub struct QueryTypeInfo {
    name: String,
    fields: Vec<QueryTypeInfo>
}

impl<S> GraphQLType<S> for QueryTypeInfo
where
    S: ScalarValue
{
    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(info.name.as_str())
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = info
            .fields
            .iter()
            .map(|field| {
                println!("B: {:?}", field);
                registry.field::<String>(field.name.as_str(), &())
            })
            .collect::<Vec<_>>();

        registry
            .build_object_type::<Query>(info, &fields)
            .into_meta()
    }
}

impl<S> GraphQLValue<S> for QueryTypeInfo
where
    S: ScalarValue
{
    type Context = ();
    type TypeInfo = QueryTypeInfo;

    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }
}

fn upper(s: &String) -> String {
    s[0..1].to_uppercase() + &s[1..]
}