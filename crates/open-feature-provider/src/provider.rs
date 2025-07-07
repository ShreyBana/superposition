use derive_builder::Builder;
use superposition_rust_sdk::{config::Token, error::BoxError, Client, Config};
use futures::future::FutureExt;
use crate::refresh::{PollStrategy, PollingRefresh, RefreshJob, RefreshStrategy};

#[non_exhaustive]
#[derive(Builder)]
pub struct ProviderOptions<R: RefreshStrategy> {
    pub endpoint: String,
    pub org_id: String,
    pub workspace_id: String,
    pub token: String,
    pub refresh_strategy: R,
}

impl<R: RefreshStrategy> ProviderOptions<R> {
    // Convenience method.
    pub fn builder() -> ProviderOptionsBuilder<R> {
        ProviderOptionsBuilder::<R>::default()
    }
}

// type ConfigOutput = ();

pub struct SuperpositionProvider<R>
where
    R: RefreshStrategy,
{
    client: Client,
    config_refresh: R::ConfigRefreshJob,
    // NOTE If we inline the required options, we could avoid some cloning.
    pub options: ProviderOptions<R>,
}

impl<R: RefreshStrategy> SuperpositionProvider<R> {
    pub fn new(options: ProviderOptions<R>) -> Self {
        let conf = Config::builder()
            .endpoint_url(options.endpoint.as_str())
            .bearer_token(Token::new(options.token.as_str(), None))
            .behavior_version_latest()
            .build();
        let client = Client::from_conf(conf);
        let c1 = client.clone();
        let oid = options.org_id.clone();
        let wid = options.workspace_id.clone();
        let crefresh = Box::new(move || {
            c1.get_config()
                .org_id(oid.clone())
                .workspace_id(wid.clone())
                .send()
                .map(|r| r.map_err(|e| Into::<BoxError>::into(e)))
                .into_future()
                .boxed()
        });
        SuperpositionProvider {
            client: client.clone(),
            config_refresh: options.refresh_strategy.new_config_refresh(crefresh),
            options,
        }
    }
}

// pub fn new_polling_provider(opts: ProviderOptions<PollStrategy>) -> SuperpositionProvider<PollingRefresh> {
//     SuperpositionProvider::<PollingRefresh>::new(opts)
// }

// #[cfg(test)]
// mod tests {
//     use std::time::Duration;

//     use crate::refresh::{PollStrategy, PollingRefresh};

//     use super::{ProviderOptions, SuperpositionProvider};

//     #[test]
//     fn check_creation() {
//         let opts = ProviderOptions::<PollStrategy>::builder()
//             .endpoint("http://localhost:8080".to_string())
//             .token("my-token".to_string())
//             .org_id("localorg".to_string())
//             .workspace_id("test".to_string())
//             .refresh_strategy(PollStrategy {
//                 timeout: Duration::from_millis(100),
//                 interval: Duration::from_millis(100),
//             })
//             .build()
//             .unwrap();
//         SuperpositionProvider::<PollingRefresh>::new(opts);
//     }
// }
