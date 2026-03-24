use service_utils::{DbConnection, helpers::WebhookData};

pub struct ExecutionContext {
    connection: DbConnection,
    workspace: String,
}

pub(crate) enum Access {
    R,
    W,
    RW,
}

pub(crate) enum Resource {
    Dimension,
    DefaultConfig,
    Context,
}

pub(crate) trait Operation<T> {
    type Input;
    const AUTHORIZATION: [(Access, Resource)];
    fn run(&self, ex_ctx: ExecutionContext, input: Self::Input) -> Result<T, String>;
}
