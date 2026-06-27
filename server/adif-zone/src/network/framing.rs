use anyhow::Context;
use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use adif_proto::adif::Packet;

pub async fn read_packet(stream: &mut TcpStream) -> anyhow::Result<Option<Packet>> {
    let len = match stream.read_u32().await {
        Ok(n) => n as usize,
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e).context("Failed to read packet length"),
    };

    if len == 0 || len > 1024 * 1024 {
        anyhow::bail!("Invalid packet length: {len}");
    }

    let mut buf = vec![0u8; len];
    stream
        .read_exact(&mut buf)
        .await
        .context("Failed to read packet body")?;

    let packet = Packet::decode(&buf[..]).context("Failed to decode protobuf packet")?;
    Ok(Some(packet))
}

pub async fn write_packet(stream: &mut TcpStream, packet: &Packet) -> anyhow::Result<()> {
    let buf = packet.encode_to_vec();
    stream
        .write_u32(buf.len() as u32)
        .await
        .context("Failed to write packet length")?;
    stream
        .write_all(&buf)
        .await
        .context("Failed to write packet body")?;
    stream.flush().await.context("Failed to flush")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use adif_proto::adif;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn round_trip_packet() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let send_task = tokio::spawn(async move {
            let mut stream = TcpStream::connect(addr).await.unwrap();
            let packet = Packet {
                sequence: 1,
                timestamp: 42,
                payload: Some(adif::packet::Payload::Heartbeat(adif::Heartbeat {
                    session_id: 99,
                })),
            };
            write_packet(&mut stream, &packet).await.unwrap();
        });

        let (mut conn, _) = listener.accept().await.unwrap();
        let packet = read_packet(&mut conn).await.unwrap().unwrap();

        assert_eq!(packet.sequence, 1);
        assert_eq!(packet.timestamp, 42);
        match packet.payload {
            Some(adif::packet::Payload::Heartbeat(h)) => assert_eq!(h.session_id, 99),
            _ => panic!("Expected Heartbeat"),
        }

        send_task.await.unwrap();
    }
}
