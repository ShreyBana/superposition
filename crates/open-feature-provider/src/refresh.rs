use std::time::Duration;

use futures::future::{BoxFuture, LocalBoxFuture};
use open_feature::async_trait;
use superposition_rust_sdk::{error::BoxError, operation::get_config::{GetConfig, GetConfigOutput}};

// trait RefreshAction {
//     type Output;
//     fn refresh(&self) -> Self::Output;
// }

type RefreshAction<O> = Box<dyn Fn() -> BoxFuture<'static, Result<O, BoxError>>>;

pub(crate) trait RefreshJob {
    type Output;
    fn output(&self) -> Option<Self::Output>;
}

pub struct PollingRefresh<O> {
    pub(crate) refresh: RefreshAction<O>
}

#[async_trait]
impl<O> RefreshJob for PollingRefresh<O> {
    type Output = O;
    async fn output(&self) -> Option<Self::Output> {
        let r = (self.refresh)().await;
        match r {
            Ok(v) => Some(v),
            _ => None
        }
    }
}

// type ConfigOutput = ();

// impl RefreshAction for ConfigAction {
//     type Output = ();
//     fn refresh(&self) -> Self::Output {
//         ()
//     }
// }
// type Experiments = ();

pub(crate) trait RefreshStrategy: Clone {
    type ConfigRefreshJob: RefreshJob<Output = GetConfigOutput>;
    // type ExperimentsRefreshJob: RefreshJob<Output = Experiments>;
    fn timeout(&self) -> Duration;
    fn new_config_refresh(
        &self,
        action: RefreshAction<GetConfigOutput>
    ) -> Self::ConfigRefreshJob;
}

#[derive(Clone)]
pub struct PollStrategy {
    pub timeout: Duration,
    pub interval: Duration,
}

impl RefreshStrategy for PollStrategy {
    type ConfigRefreshJob = PollingRefresh<GetConfigOutput>;

    fn timeout(&self) -> Duration {
        self.timeout
    }

    fn new_config_refresh(
        &self,
        f: RefreshAction<GetConfigOutput>
    ) -> Self::ConfigRefreshJob {
        PollingRefresh { refresh: f }
    }
    //     fn configure_job(action: F) -> Self::Job {
    //         PollingRefresh { fetch: action }
    //     }
}
