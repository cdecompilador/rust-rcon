extern crate rcon;


fn main() {
    let address = "localhost:25575";
    let mut conn = rcon::Connection::connect(address, "test").unwrap();

    demo(&mut conn, "list");
    demo(&mut conn, "say Rust lang rocks! ;P");
    demo(&mut conn, "save-all");
    //demo(&mut conn, "stop");
}

fn demo(conn: &mut rcon::Connection, cmd: &str) {
    let resp = conn.cmd(cmd).unwrap();
    println!("{}", resp);
}