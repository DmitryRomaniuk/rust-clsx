# rust-clsx

Ported from the [clsx](https://github.com/lukeed/clsx) JavaScript library.

## Install

```toml
// Cargo.toml
[dependencies]
# this is the development version of Yew
clsx = 0.1.0
```

## Usage

### Example using [Yew](https://yew.rs/)
```rust
<div class=clsx(vec![("class", condition), ("multiple classes", different_condition)]) />
```

## API
### clsx(input)

Returns: `String`

#### input

Type: `vec![class, condition]`

