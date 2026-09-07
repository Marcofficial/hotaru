use hotaru_core::protocol::EndpointOutcome;

use crate::message::HyperResponse;
use crate::traits::protocol_error::H2perError;
use crate::traits::request_context::HyperContext;

/// Lets handlers return a complete Hyper response.
impl<const VERSION: u8> EndpointOutcome<HyperContext<VERSION>> for HyperResponse {
    fn apply_to(self, context: &mut HyperContext<VERSION>) -> Result<(), H2perError> {
        context.set_response(self);
        Ok(())
    }
}

/// Lets handlers return an owned text body.
impl<const VERSION: u8> EndpointOutcome<HyperContext<VERSION>> for String {
    fn apply_to(self, context: &mut HyperContext<VERSION>) -> Result<(), H2perError> {
        context.set_response(HyperResponse::text(self));
        Ok(())
    }
}

/// Lets handlers return a static text body.
impl<const VERSION: u8> EndpointOutcome<HyperContext<VERSION>> for &'static str {
    fn apply_to(self, context: &mut HyperContext<VERSION>) -> Result<(), H2perError> {
        context.set_response(HyperResponse::text(self));
        Ok(())
    }
}
