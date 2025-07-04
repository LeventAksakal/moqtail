use super::constant::{ControlMessageType, SubscribeErrorCode};
use super::control_message::ControlMessageTrait;
use crate::model::common::reason_phrase::ReasonPhrase;
use crate::model::common::varint::{BufMutVarIntExt, BufVarIntExt};
use crate::model::error::ParseError;
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug, PartialEq, Clone)]
pub struct SubscribeError {
  pub request_id: u64,
  pub error_code: SubscribeErrorCode,
  pub reason_phrase: ReasonPhrase,
  pub track_alias: u64,
}

impl SubscribeError {
  pub fn new(
    request_id: u64,
    error_code: SubscribeErrorCode,
    reason_phrase: ReasonPhrase,
    track_alias: u64,
  ) -> Self {
    Self {
      request_id,
      error_code,
      reason_phrase,
      track_alias,
    }
  }
}

impl ControlMessageTrait for SubscribeError {
  fn serialize(&self) -> Result<Bytes, ParseError> {
    let mut buf = BytesMut::new();
    buf.put_vi(ControlMessageType::SubscribeError)?;

    let mut payload = BytesMut::new();
    payload.put_vi(self.request_id)?;
    payload.put_vi(self.error_code as u64)?;
    payload.extend_from_slice(&self.reason_phrase.serialize()?);
    payload.put_vi(self.track_alias)?;

    let payload_len: u16 = payload
      .len()
      .try_into()
      .map_err(|e: std::num::TryFromIntError| ParseError::CastingError {
        context: "SubscribeOk::serialize(payload_length)",
        from_type: "usize",
        to_type: "u16",
        details: e.to_string(),
      })?;

    buf.put_u16(payload_len);
    buf.extend_from_slice(&payload);

    Ok(buf.freeze())
  }

  fn parse_payload(payload: &mut Bytes) -> Result<Box<Self>, ParseError> {
    let request_id = payload.get_vi()?;
    let error_code_raw = payload.get_vi()?;

    let error_code = SubscribeErrorCode::try_from(error_code_raw)?;

    let reason_phrase = ReasonPhrase::deserialize(payload)?;

    let track_alias = payload.get_vi()?;

    Ok(Box::new(SubscribeError {
      request_id,
      error_code,
      reason_phrase,
      track_alias,
    }))
  }

  fn get_type(&self) -> ControlMessageType {
    ControlMessageType::SubscribeError
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use bytes::Buf;

  #[test]
  fn test_roundtrip() {
    let request_id = 160669;
    let error_code = SubscribeErrorCode::Timeout;
    let reason_phrase = ReasonPhrase::try_new("They see me rollin'".to_string()).unwrap();
    let track_alias = 666;
    let subscribe_error = SubscribeError {
      request_id,
      error_code,
      reason_phrase,
      track_alias,
    };
    let mut buf = subscribe_error.serialize().unwrap();
    let msg_type = buf.get_vi().unwrap();
    assert_eq!(msg_type, ControlMessageType::SubscribeError as u64);
    let msg_length = buf.get_u16();
    assert_eq!(msg_length as usize, buf.remaining());
    let deserialized = SubscribeError::parse_payload(&mut buf).unwrap();
    assert_eq!(*deserialized, subscribe_error);
    assert!(!buf.has_remaining());
  }

  #[test]
  fn test_excess_roundtrip() {
    let request_id = 160669;
    let error_code = SubscribeErrorCode::Timeout;
    let reason_phrase = ReasonPhrase::try_new("They see me rollin'".to_string()).unwrap();
    let track_alias = 666;
    let subscribe_error = SubscribeError {
      request_id,
      error_code,
      reason_phrase,
      track_alias,
    };
    let serialized = subscribe_error.serialize().unwrap();
    let mut excess = BytesMut::new();
    excess.extend_from_slice(&serialized);
    excess.extend_from_slice(&[9u8, 1u8, 1u8]);
    let mut buf = excess.freeze();

    let msg_type = buf.get_vi().unwrap();
    assert_eq!(msg_type, ControlMessageType::SubscribeError as u64);
    let msg_length = buf.get_u16();

    assert_eq!(msg_length as usize, buf.remaining() - 3);
    let deserialized = SubscribeError::parse_payload(&mut buf).unwrap();
    assert_eq!(*deserialized, subscribe_error);
    assert_eq!(buf.chunk(), &[9u8, 1u8, 1u8]);
  }

  #[test]
  fn test_partial_message() {
    let request_id = 160669;
    let error_code = SubscribeErrorCode::Timeout;
    let reason_phrase = ReasonPhrase::try_new("They see me rollin'".to_string()).unwrap();
    let track_alias = 666;
    let subscribe_error = SubscribeError {
      request_id,
      error_code,
      reason_phrase,
      track_alias,
    };
    let mut buf = subscribe_error.serialize().unwrap();
    let msg_type = buf.get_vi().unwrap();
    assert_eq!(msg_type, ControlMessageType::SubscribeError as u64);
    let msg_length = buf.get_u16();
    assert_eq!(msg_length as usize, buf.remaining());

    let upper = buf.remaining() / 2;
    let mut partial = buf.slice(..upper);
    let deserialized = SubscribeError::parse_payload(&mut partial);
    assert!(deserialized.is_err());
  }
}
