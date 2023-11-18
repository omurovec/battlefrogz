# Zupass BattleFrogz

## Overview

This is a hackathon project for the ETHGlobal Istanbul 2023 hackathon which extends the Zupass FrogCrypto PCDs to allow for 2PC battling of your frogs.

What makes this project interesting is that we use the [tandem](https://github.com/sine-fdn/tandem) MPC crate so that for each battle, neither party gets to know the other party's frog, even after the battle is finished! They are still able to verify that the battle was performed do to the nature of this 2PC scheme.

## Security Considerations

This 2PC setup ensures that the circuit is performed trustlessly and neither party is able to learn the other party's inputs however there is still some trust in this system. We must trust that both parties are inputting valid frog data that they own and we are trusting that the player who evaluates the final circuit will share that information with the other player.

## Further Improvements

To improve this system and make it truly trustless, the next steps would be to verify the EdDSA signature of the Frog PCDs and wrap this along with the garbling and oblivious transfer steps.

## Usage

To test out this project for yourself:

### 1. Install rust/cargo

```sh
curl https://sh.rustup.rs -sSf | sh
```

### 2. Clone and cd into this repo

```sh
git clone git@github.com:omurovec/battlefrogz.git
cd battlefrogz
```

### 3. Install and start the tandem server

```sh
cargo install --features="bin" --git https://github.com/sine-fdn/tandem.git tandem_http_server
tandem_http_server
```

### 4. Run the client

```sh
cargo run
```
