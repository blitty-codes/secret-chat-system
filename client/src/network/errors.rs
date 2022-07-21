#[derive(Debug)]
pub enum ConnectionErrors {
	FailedConnection,
	FailedHandshake,
	FailedSendString
}
