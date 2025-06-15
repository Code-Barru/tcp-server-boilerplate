## Encryption Request

Packet ID : `0x01`

Bound to `Agent`

Data Sent

| Field        | Type    | Size (bytes) | Description                    |
| ------------ | ------- | ------------ | ------------------------------ |
| Key          | bytes[] | 32           | The public DH Key of the Agent |
| verify_token | u64     | 8            | A u64 of random bytes          |
