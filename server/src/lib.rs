#[allow(warnings)]
mod bindings;

use crate::bindings::exports::docs::validator::form::Guest;
use bindings::docs::validator::validate::{validate_email, validate_phone};
struct Component;

impl Guest for Component {
    /// Say hello!
    fn validate(email: String, phone: String) -> bool {
        validate_email(&email) && validate_phone(&phone)
    }
}

bindings::export!(Component with_types_in bindings);
