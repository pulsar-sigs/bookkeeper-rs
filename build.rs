fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/protos")
        .inputs(&[
            "protos/BookkeeperProtocol.proto",
            "protos/DataFormats.proto",
            "protos/DbLedgerStorageDataFormats.proto",
        ])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}
