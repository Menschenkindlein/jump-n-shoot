# Jump & Shoot

A game of jumping and shooting.

## Controls

**W** - left player jump
**D** - left player shoot

**Up-arrow** - right player jump
**Left-arrow** - right player shoot

## Gameflow

In the **Lobby** you should press any controls of the respective players to notify that this player is ready. After all are ready, the game transitions into **Get Ready** state. There you have some time to collect yourself before the battle begins. After the progress bar disappears the **Game** starts: everybody can start jumping and shooting. After at least one bullet reaches the target, the game ends, and the killed square is marked with a darker color. To transition to **Lobby** again, you should press **Space**.

## Build

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run`
