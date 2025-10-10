use shared::packets::{FileChunk, Frame, Packet, PacketType, PingRequest, PongResponse};

pub fn dispatch_frame(frame: &Frame) -> Vec<Frame> {
    match frame.packet_type {
        PacketType::PingRequest => vec![handle_ping(frame)],
        PacketType::FileChunkTest => handle_file_chunk_test(frame),
        _ => {
            eprintln!("Unhandled packet type: {:?}", frame.packet_type);
            vec![Frame::new(frame.request_id, frame.packet_type, vec![])]
        }
    }
}

fn handle_ping(frame: &Frame) -> Frame {
    let ping_request = match PingRequest::deserialize(&frame.payload) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to deserialize PingRequest: {}", e);
            return Frame::new(frame.request_id, PacketType::PongResponse, vec![]);
        }
    };

    let pong_response = PongResponse::new(ping_request.timestamp);

    let payload = match pong_response.serialize() {
        Ok(data) => data[1..].to_vec(),
        Err(e) => {
            eprintln!("Failed to serialize PongResponse: {}", e);
            vec![]
        }
    };

    Frame::new(frame.request_id, PacketType::PongResponse, payload)
}

fn handle_file_chunk_test(frame: &Frame) -> Vec<Frame> {
    println!("Received FileChunkTest request, sending 5 chunks...");

    let total_chunks = 5;
    let chunk_size = 1024;
    let test_data: Vec<u8> = (0..chunk_size).map(|i| (i % 256) as u8).collect();

    let mut frames = Vec::new();

    for i in 0..total_chunks {
        let chunk = FileChunk::new(i, test_data.clone());
        let payload = match chunk.serialize() {
            Ok(data) => data[1..].to_vec(),
            Err(e) => {
                eprintln!("Failed to serialize chunk {}: {}", i, e);
                continue;
            }
        };

        let is_last = i == total_chunks - 1;
        let response_frame = Frame::new_with_flag(
            frame.request_id,
            PacketType::FileChunkTest,
            is_last,
            payload,
        );

        frames.push(response_frame);
        println!("  Prepared chunk {}/{} (is_last: {})", i + 1, total_chunks, is_last);
    }

    frames
}
