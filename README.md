### Clean store cache
```
anchor clean
```

### Sync keys
```
anchor keys sync
```

### Build the app

```
anchor build
```


### Running a local node

```
solana-test-validator
```

You can pass -r to reset before start

### Deploy

```
anchor deploy
```

#### Set network

```
solana config set -ul
```
You can set `-ul` for local, and `-ud` for devnet. `-um` set mainnet.

#### Options

`--provider.cluster <NETWORK>` can switch the network. You can pass `localnet` or `devnet` or `mainnet`

``
