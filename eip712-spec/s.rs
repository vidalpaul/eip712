pub trait S {
    const TYPE_NAME: &'static str;
}

impl S for crate::EIP712Domain {
    const TYPE_NAME: &'static str = "EIP712Domain";
}
