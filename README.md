# Traffic Light

## Usage

Assuming you have Rust and Cargo installed. If not, see https://rustup.rs/.

### Running Tests
Tests can be found in `src/lib.rs`.

```
$ cargo test
```

### Running the Application

To run the application debug release:
```
$ cargo run
```

To run the release application (no debug messages):
```
$ cargo run --release
```

### Building Docs

To build and open the auto-generated documentation in your browser:
```
$ cargo doc --no-deps --open
```

## TODOs
- ~~Cycle the light between states based on time passed.~~
- ~~Create intersection with two traffic lights in compatible states (East-West light and North-South light).~~
- ~~Handle "button" press on on North-South light to change to _red_.~~
  > The push-button causes the North-South light to change immediately if it has been green for more than 30 seconds. If less than 30 seconds have elapsed, the light will change once it has been green for 30 seconds.
- Write tests.
    - For verifying button press functionality and it's effect on state.
    