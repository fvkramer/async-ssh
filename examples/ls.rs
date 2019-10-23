extern crate async_ssh;
extern crate futures;
extern crate mio;
extern crate tokio_core;
extern crate tokio_io;

use async_ssh::Session;
use futures::Future;
use mio::TcpStream;
use std::io::prelude::*;

fn main() {
    //Connect to local SSH server
    let ls_out  = TcpStream::conect("127.0.0.1:22")
        .and_then(Session::new)
        .and_then(|session| session.authenticate_key("ec2-users", key))
        .and_then(|session| {
            session
                .channel_open()
                .and_then(|channel| channel.exec("ls"))
                .and_then(|channel| tokio_io::io::read_to_end(channel, Vec::new())
                .and_then(|(channel, data)| channel.exit_status().map(move |status| (status, data)))
        });

    let core = tokio::core::reactor::Core::new().unwrap();
    let (status, data) = core.run(ls_out).unwrap();
    println!("{}", data);
    println!("{}", status);
}
