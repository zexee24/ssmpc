pub enum Message {
    Play,
    Pause,
    Skip(Vec<u64>),
}
