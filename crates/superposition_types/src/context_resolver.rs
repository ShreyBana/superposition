use serde_json::{Map, Value};
use std::ops::Deref;

use crate::{result, Context};

type Dimensions = Map<String, Value>;

trait ContextResolver {
    /// Returns `owned` dimensions.
    fn get_dimensions(&self, ctx: &Context) -> result::Result<Dimensions>;
    fn apply(&self, condition: &Map<String, Value>, context: &Map<String, Value>)
        -> bool;
}

struct JSONLogicResolver;

impl ContextResolver for JSONLogicResolver {
    fn get_dimensions(&self, ctx: &Context) -> result::Result<Dimensions> {
        // TODO use `fn extract_dimensions`
        Ok(Map::default())
    }

    fn apply(
        &self,
        condition: &Map<String, Value>,
        context: &Map<String, Value>,
    ) -> bool {
        let cond = Value::Object(condition.clone());
        let ctx = Value::Object(context.clone());
        jsonlogic::apply(&cond, &ctx)
            == Ok(Value::Bool(true))
    }
}

struct SuperpositionResolver;

impl ContextResolver for SuperpositionResolver {
    fn get_dimensions(&self, ctx: &Context) -> result::Result<Dimensions> {
        Ok(ctx.condition.deref().clone())
    }

    fn apply(
        &self,
        condition: &Map<String, Value>,
        context: &Map<String, Value>,
    ) -> bool {
        crate::apply(condition, context)
    }
}
