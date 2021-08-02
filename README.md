# rust-clsx

## How to use in yew framework

```
use std::collections::HashMap;

let mut classes_mic = HashMap::new();
classes_mic.insert("fa", true);
classes_mic.insert("fa-microphone", self.state.enable_mic);
classes_mic.insert("fa-microphone-slash background__red", !self.state.enable_mic);

<i class=clsx_h(classes_mic) />

<i class=clsx(Box::new([("fa", true), ("fa-video-camera", self.state.enable_camera), ("fa-video-slash background__red", !self.state.enable_camera)])) />

```