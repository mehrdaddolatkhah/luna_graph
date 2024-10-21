use async_graphql::Object;

#[derive(Default)]
pub struct UserQuery;

impl UserQuery {
    async fn say_hello(&self) -> &str {
        "Hello World!"
    }
}

#[derive(Default)]
pub struct UserMutation;
