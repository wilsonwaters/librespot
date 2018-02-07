#![crate_name = "librespot"]

#![cfg_attr(feature = "cargo-clippy", allow(unused_io_amount))]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate base64;
extern crate crypto;
extern crate futures;
extern crate hyper;
extern crate num_bigint;
extern crate protobuf;
extern crate rand;
extern crate tokio_core;
extern crate url;

pub extern crate librespot_audio as audio;
pub extern crate librespot_core as core;
pub extern crate librespot_protocol as protocol;
pub extern crate librespot_metadata as metadata;

#[cfg(feature = "alsa-backend")]
extern crate alsa;

#[cfg(feature = "portaudio-rs")]
extern crate portaudio_rs;

#[cfg(feature = "libpulse-sys")]
extern crate libpulse_sys;

#[cfg(feature = "libc")]
extern crate libc;

#[cfg(feature = "with-dns-sd")]
extern crate dns_sd;

#[cfg(not(feature = "with-dns-sd"))]
#[cfg(not(target_os="windows"))]
extern crate mdns;

pub mod audio_backend;

#[cfg(not(target_os="windows"))]
pub mod discovery;
pub mod keymaster;
pub mod mixer;
pub mod player;

pub mod lms;

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
