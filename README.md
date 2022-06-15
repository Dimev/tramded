# Tramded
A webhook server to check if there's an issue with U-OV Tram 22
Written in rust to (hopefully) have a higher uptime than tram 22 itself

## How it works
 - every N seconds, it makes a request to https://ovapi.nl/ to see what trams are going
 - if there are any alerts, it will parse those and see which ones are relevant
 - it will then forward them to the given webhook

## Why
Why not

## Licence
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.