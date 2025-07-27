use std::{convert::Infallible, pin::Pin, sync::Arc};

use bytes::Bytes;
use futures::Stream;
use tokio::sync::Mutex;

use crate::app::model::AppState;

use super::Processor;

pub struct OpenaiProcessor<S: Stream<Item = Result<Bytes, reqwest::Error>> + Send + Unpin>(
    Processor<S>,
);

impl<S> OpenaiProcessor<S>
where
    S: Stream<Item = Result<Bytes, reqwest::Error>> + Send + Unpin + 'static,
{
    pub fn new(input_stream: S, app_state: Arc<Mutex<AppState>>) -> Self {
        OpenaiProcessor(Processor::new(
            input_stream,
            app_state,
            Arc::new(Box::pin(|data, state| async { Some(Ok(data)) })),
        ))
    }
}

async fn processor(data: Bytes, _state: Arc<Mutex<AppState>>) -> Option<Result<Bytes, Infallible>> {
    Some(Ok(data))
}
