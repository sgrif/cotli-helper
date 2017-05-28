COTLI Helper
==

Shitty tool to tell you the best formation in Crusaders of the Lost Idols. Very
WIP. I mostly built this to play around with Monte Carlo Tree Searches, but you
may find the tool useful.

![](http://i.imgur.com/MEU72CF.gif)

Limitations
--

- Only primary DPS is considered. This means the tool ignores gold buffs, click
  damage, and abilities like Merci's deflect evil. The only number it cares
  about is the total formation DPS.
- Passive Criticals is not considered
- Many crusaders are not yet implemented
- No event formations
- Things like weekend buffs, item buffs, experience points, and storm rider are
  not supported

How to Run
--

There is currently no pre-distributed binary. These instructions are for running
form source, and assume you already have a Rust toolchain installed.

Copy `example_user_data.toml` to `user_data.toml`, and fill the file with your
data. After populating that file, run `cargo build --release`, and then
`target/release/cotli-helper --help` to see the available options.
