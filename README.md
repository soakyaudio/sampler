# sampler
This project aims to be a full-fledged software sampler written in Rust.
While I initially used Apple's AUSampler for sampled instruments in my iOS app [soundboard](https://apps.apple.com/app/soundboard-create-music/id1619264410), its largely undocumented behavior and [mysterious bugs](http://openradar.appspot.com/radar?id=5598760801402880) called for a better alternative.

_sampler_ is open source, so feel free to contribute and to use the code in your own projects :).
It shall also serve as a larger example of using Rust for real-time audio processing.

## Contents
- [Build Instructions](#build-instructions)
- [Features](#features)
- [Roadmap](#roadmap)
- [Third-Party Credits](#third-party-credits)
- [License](#license)

## Build Instructions
Active development currently happens on macOS only.
There are few things that might fall apart on other systems, so any help with cross-platform support is appreciated :).

0. Prerequisites
    - Install [Rust](https://www.rust-lang.org/tools/install)
1. Clone the repo
    ```sh
    # clone via ssh
    git clone git@github.com:soakyaudio/sampler.git

    # or via https
    git clone https://github.com/soakyaudio/sampler.git
    ```
2. Build and start the app
    ```sh
    cd sampler
    cargo run
    ```
3. _Optional commands_
    ```sh
    # run unit tests
    cargo test

    # build documentation
    cargo doc
    ```

## Features
- Standalone wrapper app with real-time audio output & midi input
- Polyphonic sine-wave instrument with linear ADSR envelope
- [Here's some music](https://open.spotify.com/track/24LugbAAG8AIJGOLu52iOv?si=baf4c8c1f1fd498c) while you wait for more features...


## Roadmap
### 0.1.0
- [x] Code style, rustfmt
- [x] Fix compiler warnings
- [x] Multiple velocities
- [ ] SFZ format
- [x] Unit tests
- [x] Voice stealing
- [x] WAV samples

### 0.2.0
- [ ] Looped samples
- [ ] Round robin

### 0.3.0
- [ ] Disk streaming
- [ ] Filter per voice

### 0.4.0
- [ ] AUv3 plugin wrapper

### Ideas
- [ ] FX automation / routings
- [ ] Get rid of `Send` trait and replace `Arc` with lifetimes if possible
- [ ] Setup CI/CD pipeline with GitHub Actions
- [ ] Write contribution guide

## Third-Party Credits
- [cpal](https://github.com/RustAudio/cpal) ([Apache 2.0](https://github.com/RustAudio/cpal/blob/1ac8f1549f41001acd0acef2be9214ab72e61d11/LICENSE)): Cross-platform audio I/O library in pure Rust.
- [hound](https://github.com/ruuda/hound) ([Apache 2.0](https://github.com/ruuda/hound/blob/02e66effb33683dd6acb92df792683ee46ad6a59/license)): A wav encoding and decoding library in Rust.
- [midir](https://github.com/Boddlnagg/midir/) ([MIT](https://github.com/Boddlnagg/midir/blob/c6aa24867aedee1e02284c5bb6062648f594632d/LICENSE)): Cross-platform realtime MIDI processing in Rust.
- [ringbuf](https://github.com/agerasev/ringbuf) ([MIT](https://github.com/agerasev/ringbuf/blob/939b3338a2faf8d1d490eaa9eb50a8ae02136701/LICENSE-MIT)): Lock-free SPSC FIFO ring buffer with direct access to inner data.
- [sofiza](https://github.com/andamira/sofiza) ([MIT](https://github.com/andamira/sofiza/blob/0d4ed41be0201839ef82fbc5f0702d1b2c394e18/LICENSE-MIT)): An SFZ format parser.
- And thanks to all the unmentioned thousands of contributers behind the amazing projects that keep this software running.

## License
This project is released under the MIT License.
```
MIT License

Copyright (c) 2022 Micha Hanselmann

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
