use game_of_life::ascii_buffer::FrameBuffer;

fn main() {
    let frame_buffer = FrameBuffer::new(32, 32);
    frame_buffer.draw();
}
