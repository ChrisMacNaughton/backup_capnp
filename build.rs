extern crate capnpc;

fn main() {
    ::capnpc::compile("schema", &["schema/backup.capnp"]).expect("compiling");
}
