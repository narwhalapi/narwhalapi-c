use crate::{meta::Model, transport::BoxFuture, Error, Request};
use futures_core::Stream;
use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
};
#[doc(hidden)]
pub trait Page: Model {
    type Item: Model;
    fn into_page(self) -> Result<(Vec<Self::Item>, Option<String>), Error>;
}
/// Lazy async pagination. Dropping the stream cancels an in-flight page request.
#[non_exhaustive]
pub struct PageStream<P: Page> {
    request: Request<P>,
    items: VecDeque<P::Item>,
    pending: Option<BoxFuture<Result<P, Error>>>,
    done: bool,
}
impl<P: Page> PageStream<P> {
    pub(crate) fn new(request: Request<P>) -> Self {
        Self {
            request,
            items: VecDeque::new(),
            pending: None,
            done: false,
        }
    }
}
impl<P: Page> Stream for PageStream<P> {
    type Item = Result<P::Item, Error>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            if let Some(item) = this.items.pop_front() {
                return Poll::Ready(Some(Ok(item)));
            }
            if this.done {
                return Poll::Ready(None);
            }
            if this.pending.is_none() {
                let request =
                    Request::<P>::new(this.request.client.clone(), this.request.data.clone());
                this.pending = Some(Box::pin(request.send()));
            }
            let page = match this
                .pending
                .as_mut()
                .expect("pending page")
                .as_mut()
                .poll(cx)
            {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(result) => result,
            };
            this.pending = None;
            match page.and_then(Page::into_page) {
                Err(e) => {
                    this.done = true;
                    return Poll::Ready(Some(Err(e)));
                }
                Ok((items, cursor)) => {
                    this.items = items.into();
                    match cursor {
                        Some(c) => this.request.data.cursor(&c),
                        None => this.done = true,
                    }
                    if this.items.is_empty() && !this.done {
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                }
            }
        }
    }
}
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
#[non_exhaustive]
pub struct PageIterator<P: Page> {
    request: Request<P, crate::blocking::Client>,
    items: VecDeque<P::Item>,
    done: bool,
}
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
impl<P: Page> PageIterator<P> {
    pub(crate) fn new(request: Request<P, crate::blocking::Client>) -> Self {
        Self {
            request,
            items: VecDeque::new(),
            done: false,
        }
    }
}
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
impl<P: Page> Iterator for PageIterator<P> {
    type Item = Result<P::Item, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.items.pop_front() {
                return Some(Ok(item));
            }
            if self.done {
                return None;
            }
            match self
                .request
                .client
                .send::<P>(self.request.data.clone())
                .and_then(Page::into_page)
            {
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
                Ok((items, cursor)) => {
                    self.items = items.into();
                    match cursor {
                        Some(c) => self.request.data.cursor(&c),
                        None => self.done = true,
                    }
                }
            }
        }
    }
}
