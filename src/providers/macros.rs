#[macro_export]
macro_rules! rpc_method {
    ($rpc:expr, $ret:ty) => {
        paste! {
            pub async fn [<$rpc:snake>](&self) -> ProviderResult<$ret> {
                self.inner.request(&$rpc.to_string(), rpc_params![]).await
            }
        }
    };
    ($rpc:expr, $param_name:ident, $param_type:ty, $ret:ty) => {
        paste! {
            pub async fn [<$rpc:snake>](&self, $param_name: $param_type) -> ProviderResult<$ret> {
                self.inner.request(&$rpc.to_string(), rpc_params![$param_name]).await
            }
        }
    };
    ($rpc:expr, $param1_name:ident, $param1_type:ty, $param2_name:ident, $param2_type:ty, $ret:ty) => {
        paste! {
            pub async fn [<$rpc:snake>](&self, $param1_name: $param1_type, $param2_name: $param2_type) -> ProviderResult<$ret> {
                self.inner.request(&$rpc.to_string(), rpc_params![$param1_name, $param2_name]).await
            }
        }
    };
}
