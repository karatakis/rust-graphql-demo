use sea_schema::sqlite::{TableDef};
use juniper::{GraphQLValue, GraphQLType, ScalarValue, Registry, meta::MetaType};

#[derive(Debug)]
pub struct SingleQuery {
    pub info: SingleQueryInfo
}

impl <S> GraphQLValue<S> for SingleQuery
where
    S: ScalarValue
{
    type Context = ();
    type TypeInfo = SingleQueryInfo;

    fn type_name<'i>(&self, info: &'i SingleQueryInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }
}

impl<S> GraphQLType<S> for SingleQuery
where
    S: ScalarValue
{
    fn name(info: &SingleQueryInfo) -> Option<&str> {
        Some(info.meta.name.as_str())
    }

    fn meta<'r>(info: &SingleQueryInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = info
            .meta
            .columns
            .iter()
            .map(|field| {
                println!("B: {:?}", field);
                registry.field::<String>(field.name.as_str(), &())
            })
            .collect::<Vec<_>>();

        registry
            .build_object_type::<SingleQuery>(info, &fields)
            .into_meta()
    }
}

#[derive(Debug)]
pub struct SingleQueryInfo {
    pub name: String,
    pub meta: TableDef
}