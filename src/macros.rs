#[macro_export]
macro_rules! try_msg {
    ($op:expr, $message:expr) => (
        ($op).map_err(|e| format!($message, err=e))?
    );
    ($op:expr, $message:expr, $($key:ident=$value:expr),*) => (
        ($op).map_err(|e| format!($message, err=e, $($key=$value),*))?
    );
}

#[macro_export]
macro_rules! tuple_struct_decode {
    ($name:ident) => {

        impl ::serde::Deserialize<'static> for $name {
            fn deserialize<'x, D: ::serde::Deserializer<'x>>(d: &mut D)
                -> Result<Step, D::Error>
            {
                unimplemented!();
            }
        }
    }
}
