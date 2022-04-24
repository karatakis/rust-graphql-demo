use indexmap::IndexMap;
use juniper::{
    meta::MetaType, Arguments, EmptyMutation, EmptySubscription, ExecutionResult, Executor,
    GraphQLType,GraphQLValueAsync, GraphQLValue, Registry, RootNode, ScalarValue,
};
use sea_schema::sqlite::{Schema};

pub struct NodeTypeInfo {
    node_name: String,
    node_type: NodeType,
    node_children: Vec<NodeTypeInfo>,
}

pub enum NodeType {
    String,
    Boolean,
    Object,
}

impl<S> GraphQLType<S> for NodeTypeInfo
where
    S: ScalarValue
{
    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(&info.node_name)
    }
}


pub struct Node {
    // TODO make this thing dynamic
    attributes: IndexMap<String, bool>,
}

impl<S> GraphQLValue<S> for Node
where
    S: ScalarValue,
{
    type Context = ();
    type TypeInfo = NodeTypeInfo;

    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }

    fn resolve_field(
        &self,
        _info: &Self::TypeInfo,
        field_name: &str,
        _: &Arguments<S>,
        executor: &Executor<Self::Context, S>,
    ) -> ExecutionResult<S> {
        executor.resolve(&(), &self.attributes.get(field_name).unwrap())
    }
}

impl<S> GraphQLValueAsync<S> for Node
where
    S: ScalarValue + Send + Sync
{

}

impl<S> GraphQLType<S> for Node
where
    S: ScalarValue,
{
    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(&info.node_name)
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = info
            .node_children
            .iter()
            .map(|node| {
                match node.node_type {
                    NodeType::String => registry.field::<String>(node.node_name.as_str(), &()),
                    NodeType::Boolean => registry.field::<bool>(node.node_name.as_str(), &()),
                    // TODO
                    NodeType::Object => registry.field::<NodeTypeInfo>(node.node_name.as_str(), node),
                }

            })
            .collect::<Vec<_>>();

        registry
            .build_object_type::<Node>(info, &fields)
            .into_meta()
    }
}

pub async fn get_graphql_schema(schema: &Schema) -> RootNode<'static, Node, EmptyMutation<()>, EmptySubscription<()>> {
    let mut node_info = NodeTypeInfo {
        node_name: "Query".to_string(),
        node_type: NodeType::Object,
        node_children: Vec::new(),
    };
    let mut node = Node {
        attributes: IndexMap::new(),
    };
    for table in schema.tables.iter() {
        node_info.node_children.push(
            NodeTypeInfo {
                node_name: table.name.clone(),
                node_type: NodeType::Object,
                node_children: Vec::new(),
            }
        );
        node.attributes.insert(table.name.clone(), false);
    }
    let schema: RootNode<'_, _, _, _> = RootNode::new_with_info(
        node,
        EmptyMutation::new(),
        EmptySubscription::new(),
        node_info,
        (),
        (),
    );

    schema
}
