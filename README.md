# rust-clsx

portable js library [clsx](https://github.com/lukeed/clsx/blob/master/src/index.js)

## How to use in yew framework

```rust
<i class=clsx(vec![("fa", true), ("fa-video-camera", self.state.enable_camera), ("fa-video-slash background__red", !self.state.enable_camera)]) />
```