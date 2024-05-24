use self::prelude::*;

pub mod api_key;

pub mod prelude {
    pub use super::{api_key::ApiKey, Api};
}

pub trait Api {
    const KEY: ApiKey;

    type Request;
    type Response;
}
