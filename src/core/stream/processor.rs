// pub mod anthropic;
// pub mod openai;

use bytes::Bytes;
use futures::Stream;
use std::{
    error::Error as StdError,
    pin::Pin,
    task::{Context, Poll},
};

pin_project_lite::pin_project! {
    pub struct LlmStreamTransformer<S, E>
    where
        S: Stream<Item = Result<Bytes, E>>,
        E: StdError,
    {
        #[pin]
        stream: S,
    }
}

impl<S, E> Stream for LlmStreamTransformer<S, E>
where
    S: Stream<Item = Result<Bytes, E>> + Send + Unpin + 'static,
    E: StdError,
{
    type Item = Result<Bytes, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match this.stream.poll_next(cx) {
            Poll::Ready(Some(Ok(bytes))) =>
                if bytes.is_empty() {
                    Poll::Pending
                } else {
                    Poll::Ready(Some(Ok(bytes)))
                },
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
