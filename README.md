# pozk-2048

### Serialize
- Inputs (bytes)
```
[ bytes[], uint256[], uint256, bytes, uint256, uint256, uint256, uint256 ]
[ board, packedBoard, packedDir, direction, address, nonce, step, stepAfter ]

e.g.

{
    "board": [
        [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 4, 6, 0, 1, 2, 4, 0, 0, 0, 5, 0, 0, 1, 3]
    ],
    "packedBoard": ["35218731827200", "2515675923718842875939"],
    "packedDir": "311800516178808354245949615821275955",
    "direction": [0, 3, 3, 0, 0, 0, 3, 0, 3, 3, 0, 3, 3, 0, 3, 0, 2, 0, 3, 3, 0, 2, 0, 3, 0, 0, 3, 0, 2, 0, 3, 3, 0, 0, 3, 0, 3, 3, 0, 3, 3, 3, 3, 3, 0, 0, 3, 2, 3, 3, 0, 3, 3, 0, 0, 3, 0, 3, 0, 3],
    "address": "6789",
    "step": 0,
    "stepAfter": 60,
    "nonce": "456"
}
```

- Publics (bytes)
```
uint256[7]

publics
```

- Proof (bytes)
```
[ uint256[2], uint256[2][2], uint256[2] ]
[ proof.a, proof.b, proof.c ]

```
