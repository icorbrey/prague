use self::api_key::ApiKey;

pub mod api_key;
pub mod prelude;

pub trait Api {
    const KEY: ApiKey;

    type Request;
    type Response;
}
