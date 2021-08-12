# rust-clsx

portable js library [clsx](https://github.com/lukeed/clsx/blob/master/src/index.js)

## How to use in yew framework

```
use std::collections::HashMap;

let mut classes_mic = HashMap::new();
classes_mic.insert("fa", true);
classes_mic.insert("fa-microphone", self.state.enable_mic);
classes_mic.insert("fa-microphone-slash background__red", !self.state.enable_mic);

<i class=clsx_h(classes_mic) />

<i class=clsx(vec![("fa", true), ("fa-video-camera", self.state.enable_camera), ("fa-video-slash background__red", !self.state.enable_camera)]) />

```