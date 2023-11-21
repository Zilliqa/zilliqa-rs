#[macro_export]
macro_rules! rpc_method {
    ($rpc:expr, $ret:ty) => {
        paste! {
            pub async fn [<$rpc:snake>](&self) -> ProviderResult<$ret> {
                Ok(self.inner.request(&$rpc.to_string(), rpc_params![]).await?)
            }
        }
    };
    ($rpc:expr, $param_name:ident, $param_type:ty, $ret:ty) => {
        paste! {
            pub async fn [<$rpc:snake>](&self, $param_name: $param_type) -> ProviderResult<$ret> {
                Ok(self.inner.request(&$rpc.to_string(), rpc_params![$param_name]).await?)
            }
        }
    };
}
