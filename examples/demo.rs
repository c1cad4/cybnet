use cyb_net::mesh::MeshPacket;
fn main() {
    let p = MeshPacket {
        id: "demo".into(),
        source: "robot".into(),
        destination: "farm".into(),
        payload: b"telemetry".to_vec(),
        ttl: 3,
    };
    p.validate().unwrap();
    println!("{:?}", p.forwarded().unwrap());
}
