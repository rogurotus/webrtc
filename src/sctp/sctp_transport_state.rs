use std::fmt;

/// SCTPTransportState indicates the state of the SCTP transport.#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SCTPTransportState {
    Unspecified = 0,

    /// SCTPTransportStateConnecting indicates the SCTPTransport is in the
    /// process of negotiating an association. This is the initial state of the
    /// SCTPTransportState when an SCTPTransport is created.
    Connecting,

    /// SCTPTransportStateConnected indicates the negotiation of an
    /// association is completed.
    Connected,

    /// SCTPTransportStateClosed indicates a SHUTDOWN or ABORT chunk is
    /// received or when the SCTP association has been closed intentionally,
    /// such as by closing the peer connection or applying a remote description
    /// that rejects data or changes the SCTP port.
    Closed,
}

impl Default for SCTPTransportState {
    fn default() -> Self {
        SCTPTransportState::Unspecified
    }
}

const SCTP_TRANSPORT_STATE_CONNECTING_STR: &str = "Connecting";
const SCTP_TRANSPORT_STATE_CONNECTED_STR: &str = "Connected";
const SCTP_TRANSPORT_STATE_CLOSED_STR: &str = "Closed";

impl From<&str> for SCTPTransportState {
    fn from(raw: &str) -> Self {
        match raw {
            SCTP_TRANSPORT_STATE_CONNECTING_STR => SCTPTransportState::Connecting,
            SCTP_TRANSPORT_STATE_CONNECTED_STR => SCTPTransportState::Connected,
            SCTP_TRANSPORT_STATE_CLOSED_STR => SCTPTransportState::Closed,
            _ => SCTPTransportState::Unspecified,
        }
    }
}

impl fmt::Display for SCTPTransportState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            SCTPTransportState::Connecting => SCTP_TRANSPORT_STATE_CONNECTING_STR,
            SCTPTransportState::Connected => SCTP_TRANSPORT_STATE_CONNECTED_STR,
            SCTPTransportState::Closed => SCTP_TRANSPORT_STATE_CLOSED_STR,
            SCTPTransportState::Unspecified => crate::UNSPECIFIED_STR,
        };
        write!(f, "{}", s)
    }
}
