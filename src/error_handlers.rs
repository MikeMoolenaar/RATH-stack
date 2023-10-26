use actix_web::{dev::ServiceResponse, middleware::ErrorHandlerResponse, Result};

pub fn error_500<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    // split service response into request and response components
    let (req, res) = res.into_parts();

    // set body of response to modified body
    let res = res.set_body("An error occurred.");

    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

pub fn error_429<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    // split service response into request and response components
    let (req, res) = res.into_parts();

    // set body of response to modified body
    let res = res.set_body("Hold your horses! You are doing too many requests.");

    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
