use std::net::UdpSocket;
use bytebuffer::ByteBuffer;

pub struct NetClient<'a> {
    pub client: UdpSocket,
    pub server: &'a str,
    pub player_id: i64
}

impl<'a> NetClient<'a> {
    pub fn new() -> NetClient<'a> {
        let mut client = NetClient {
            server: "127.0.0.1:1337",
            client: UdpSocket::bind("0.0.0.0:0").unwrap(),
            player_id: -1
        };

        let mut buf = ByteBuffer::new();
        buf.write_u8(6);
        let mut response = client.write_bytes(buf);

        client.player_id = response.read_i64();

        println!("User ID is {:?}.", client.player_id);

        client
    }
    
    pub fn write_player_position(&self, pos: (f32, f32, f32)) {
        let mut buf = ByteBuffer::new();

        buf.write_u8(0);
        buf.write_i64(self.player_id);
        buf.write_f32(pos.0);
        buf.write_f32(pos.1);
        buf.write_f32(pos.2);

        self.write_bytes(buf);
    }

    fn write_bytes(&self, buf: ByteBuffer) -> ByteBuffer {
        let bytes = &buf.to_bytes();
        self.client.send_to(bytes, self.server).unwrap();

        let mut raw: [u8; 128] = [0; 128];
        self.client.recv_from(&mut raw).unwrap();
        let mut buf = ByteBuffer::from_bytes(&raw);
        if raw[0] == 127 {
            panic!("Invalid response");
        } else {
            buf.read_u8();
            return buf;
        }
    }
}