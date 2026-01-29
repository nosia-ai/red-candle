use magnus::Error;

pub fn wrap_std_err(err: Box<dyn std::error::Error + Send + Sync>) -> Error {
    let ruby = magnus::Ruby::get().unwrap();
    Error::new(ruby.exception_runtime_error(), err.to_string())
}

pub fn wrap_candle_err(err: candle_core::Error) -> Error {
    let ruby = magnus::Ruby::get().unwrap();
    Error::new(ruby.exception_runtime_error(), err.to_string())
}

pub fn wrap_hf_err(err: hf_hub::api::sync::ApiError) -> Error {
    let ruby = magnus::Ruby::get().unwrap();
    Error::new(ruby.exception_runtime_error(), err.to_string())
}
