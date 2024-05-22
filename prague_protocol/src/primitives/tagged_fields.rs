use std::collections::HashMap;

use super::field::Field;

pub struct TaggedFields {
    pub fields: HashMap<i32, Field>,
}
