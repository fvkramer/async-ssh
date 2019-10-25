extern crate futures;
extern crate thrussh;
extern crate thrussh_keys;
extern crate tokio_io;

use futures::{Future, Async};
use tokio_io::{AsyncRead, AsyncWrite};
use std::rc::Rc;
use std::cell::RefCell;


pub struct NewSession<S: AsyncRead + AsyncWrite> {
    connection: thrussh::client::Connection<S, Self>,
}

pub struct Session<S: AsyncRead + AsyncWrite> {
    connection: thrussh::client::Connection<S, Self>,
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
    pub fn channel_open(self) -> Box<Future<Item = (Session<S>, OpenedChannel<S>), Error = ()>> {}
}

pub struct OpenedChannel;

impl OpenedChannel {
    pub fn exec(self) -> Box<Future<Item = Channel, Error = ()>> {}
}

pub struct Channel<S: AsyncRead + AsyncWrite> {
    session: Rc<RefCell<Session<S>>>,
    id: thrussh::ChannelId,
}

impl Channel {
    pub fn exit_status(self) -> Box<Future<Item = u32, Error = ()>> {}
}

use std::io::prelude::*;
use std:io;

struct ChannelState {
    data: Vec<u8>,
    exit_status: u32,
}

impl Read for Channel {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
             match self.session.borrow_mut().poll() {
                Ok(Async::Reader(())) => Ok(0),
             }
        }
    }
}

impl Write for Channel {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {}
    fn flush(&mut self) -> Result<()> {}
}

impl AsyncRead for Channel {}

impl AsyncWrite for Channel {
    fn shutdown(&mut self) -> Poll<(), Error>;
}
