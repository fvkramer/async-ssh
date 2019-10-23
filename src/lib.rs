extern crate futures;
extern crate thrussh;
extern crate thrussh_keys;
extern crate tokio_io;

use futures::Future;
use tokio_io::{AsyncRead, AsyncWrite};

pub struct NewSession<S: AsyncRead + AsyncWrite> {
    connection: thrussh::client::Connection<S, Self>,
}

pub struct NewSessionFuture<S: AsyncRead + AsyncWrite> {}

impl<S: AsyncRead + AsyncWrite> Future for NewSessionFuture<S> {
    type Item = NewSession<S>;
    type Error = ();

    fn poll() {}
}

pub struct Session<S: AsyncRead + AsyncWrite> {
    connection: thrussh::client::Connection<S, Self>,
}

pub struct SessionFuture<S: AsyncRead + AsyncWrite> {}

impl<S: AsyncRead + AsyncWrite> Future for SessionFuture<S> {
    type Item = Session<S>;
    type Error = ();

    fn poll() {}
}

impl<S: AsyncRead + AsyncWrite> NewSession<S> {
    pub fn new(stream: S) -> Box<Future<Item = NewSession<S>, Error = ()>> {}

    pub fn authenticate_key(
        self,
        user: &str,
        key: thrussh_keys::key::KeyPair,
    ) -> Box<Future<Item = Session<S>, Error = ()>> {
    }
}

impl<S: AsyncRead + AsyncWrite> Session<S> {
    pub fn channel_open(self) -> Box<Future<Item = Channel, Error = ()>> {}
}

pub struct OpenedChannel;
pub struct Channel;
