## Encryption Response

Packet ID : `0x02`

Bount to `Server`

Data Sent

| Field          | Type    | Size (bytes) | Description                                                   |
| -------------- | ------- | ------------ | ------------------------------------------------------------- |
| key            | bytes[] | 32           | The public DH Key of the Agent                                |
| nonce          | bytes[] | 12           | Nonce to decrypt the token                                    |
| verified_token | bytes[] | 24           | Verify token encrypted with the same key as the shared secret |
