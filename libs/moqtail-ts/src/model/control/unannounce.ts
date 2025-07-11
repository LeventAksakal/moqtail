import { BaseByteBuffer, ByteBuffer, FrozenByteBuffer } from '../common/byte_buffer'
import { Tuple } from '../common/tuple'
import { ControlMessageType } from './constant'
import { LengthExceedsMaxError } from '../error/error'

export class Unannounce {
  constructor(public readonly trackNamespace: Tuple) {}

  getType(): ControlMessageType {
    return ControlMessageType.Unannounce
  }

  serialize(): FrozenByteBuffer {
    const buf = new ByteBuffer()
    buf.putVI(ControlMessageType.Unannounce)
    const payload = new ByteBuffer()
    payload.putTuple(this.trackNamespace)
    const payloadBytes = payload.toUint8Array()
    if (payloadBytes.length > 0xffff) {
      throw new LengthExceedsMaxError('Unannounce::serialize(payloadBytes.length)', 0xffff, payloadBytes.length)
    }
    buf.putU16(payloadBytes.length)
    buf.putBytes(payloadBytes)
    return buf.freeze()
  }

  static parsePayload(buf: BaseByteBuffer): Unannounce {
    const trackNamespace = buf.getTuple()
    return new Unannounce(trackNamespace)
  }
}

if (import.meta.vitest) {
  const { describe, test, expect } = import.meta.vitest
  describe('Unannounce', () => {
    test('roundtrip', () => {
      const trackNamespace = Tuple.fromUtf8Path('un/announce/me')
      const msg = new Unannounce(trackNamespace)
      const frozen = msg.serialize()
      const msgType = frozen.getVI()
      expect(msgType).toBe(BigInt(ControlMessageType.Unannounce))
      const msgLength = frozen.getU16()
      expect(msgLength).toBe(frozen.remaining)
      const deserialized = Unannounce.parsePayload(frozen)
      expect(deserialized.trackNamespace.equals(msg.trackNamespace)).toBe(true)
      expect(frozen.remaining).toBe(0)
    })
    test('excess roundtrip', () => {
      const trackNamespace = Tuple.fromUtf8Path('un/announce/me')
      const msg = new Unannounce(trackNamespace)
      const serialized = msg.serialize().toUint8Array()
      const excess = new Uint8Array([9, 1, 1])
      const buf = new ByteBuffer()
      buf.putBytes(serialized)
      buf.putBytes(excess)
      const frozen = buf.freeze()
      const msgType = frozen.getVI()
      expect(msgType).toBe(BigInt(ControlMessageType.Unannounce))
      const msgLength = frozen.getU16()
      expect(msgLength).toBe(frozen.remaining - 3)
      const deserialized = Unannounce.parsePayload(frozen)
      expect(deserialized.trackNamespace.equals(msg.trackNamespace)).toBe(true)
      expect(frozen.remaining).toBe(3)
      expect(Array.from(frozen.getBytes(3))).toEqual([9, 1, 1])
    })
    test('partial message', () => {
      const trackNamespace = Tuple.fromUtf8Path('un/announce/me')
      const msg = new Unannounce(trackNamespace)
      const serialized = msg.serialize().toUint8Array()
      const upper = Math.floor(serialized.length / 2)
      const partial = serialized.slice(0, upper)
      const buf = new ByteBuffer()
      buf.putBytes(partial)
      const frozen = buf.freeze()
      expect(() => {
        frozen.getVI()
        frozen.getU16()
        Unannounce.parsePayload(frozen)
      }).toThrow()
    })
  })
}
