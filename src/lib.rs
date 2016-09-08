#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}


extern crate capnp;

pub mod backup_capnp {
    include!(concat!(env!("OUT_DIR"), "/backup_capnp.rs"));
}
