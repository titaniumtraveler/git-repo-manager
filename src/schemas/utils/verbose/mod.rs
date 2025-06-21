use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::Deserialize;
use std::mem;

pub mod bool;
pub mod map;

pub trait VerboseEntry<'de>: Deserialize<'de> {
    type Short: Deserialize<'de> + JsonSchema;
    fn from_short(short: Self::Short) -> Self;
    fn transform_schema(schema: &mut Schema) {
        // This could lead to unnessary schema duplication,
        // but we don't really have a choice ...
        let mut generator = SchemaGenerator::default();

        let short = generator.subschema_for::<Self::Short>();
        let verbose = mem::take(schema);

        *schema = json_schema! {
            {
                "oneOf": [
                    short,
                    verbose,
                ]
            }

        };
    }
}
