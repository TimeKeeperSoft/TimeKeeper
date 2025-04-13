<div align="center">
    <img src="assets/logo.png" width="200">
    <h1>⏳ TimeKeeper</h1>
    <p><b>Manage your PC time efficiently!</b></p>
    <h4>
        <a href="./README_ru.md">README (Russian)</a>
        <span> | </span>
        <a href="https://timekeepersoft.github.io/screenshots.html">Screenshots</a> (site)
        <span> | </span>
        <a href="https://github.com/mskrasnov/TimeKeeper/releases">Releases</a> (GitHub)
        <span> | </span>
        <a href="https://t.me/TimeKeeperSoft">Telegram</a>
    </h4>
</div>

> **Note 0:** now TimeKeeper have English and Russian translations.
>
> **Note 1:** the Windows build of program hasn't been signed yet and I don't think I will ever sign it. I don't have time for this now, so deal with the program signature yourself.

## What is TimeKeeper?

TimeKeeper is the simplest cross-platform PC time-tracking program. It periodically reminds users to take breaks while working on their computers. It's free of unnecessary web and AI features.

![](assets/main_win.png)

## Idea

Many people do not take breaks when working at a PC. As a result, the efficiency of such work decreases due to fatigue, musculoskeletal problems due to sedentary lifestyle and all kinds of eye disorders. Consequently, users need a simple program that, adjusting to their rhythm of work, could remind them of the need to take a break from work.

The program is intended for use by people who are exposed to high visual loads and/or lead sedentary lifestyles.

## Motivation

1. I needed a cross-platform timekeeping program. Such a program should be simple, lightweight and quite functional.
2. Many people forget to take breaks while working on the computer, which is detrimental to their productivity and health. Take a break - stretch, get some fresh air and continue working. This will slow down the damage to your health.
3. I needed to practice developing desktop software in Rust. TimeKeeper is the second project, the first one was [ice](https://github.com/mskrasnov/ice).

## Benefits

1. **Offline.** TimeKeeper does not require a browser or Internet access to work.
2. **Cross-platform.** Our program works correctly in Windows and Linux operating systems and does not depend on most external libraries like Qt and GTK (and even the god-awful Electron).
3. **Minimalistic.** TimeKeeper occupies no more than 20 MB on the hard disk and is able to work even on weak PCs.
4. [TODO] **Statistics.** TimeKeeper is able to keep local statistics of work/rest cycles so that the user has an idea of how much they are working and how often they need to take breaks.
5. **Notifications.** Regardless of the OS on which TimeKeeper is running, you will receive notifications on your desktop when your work/rest cycle starts/ends.

## Installation

### Already compiled program

Go to the [Releases](https://github.com/mskrasnov/TimeKeeper) tab and download the latest *stable* release for your operating system. Currently only Windows (x86_64) and Linux (glibc x86_64) are supported.

### Build from source code

**Dependencies:**

- Rust toolchain (can be installed with `rustup`);
- **Windows:** MSVC;
- **Linux:** `binutils`, `gcc`, Xorg/Wayland

**Building in Windows:**

```ps
cargo build --release
.\target\release\time_keeper
```

**Building in Linux:**

```bash
sudo apt install build-essential -y

cargo build --release
./target/release/time_keeper
```

## Technology stack

- **OS:** Windows, Linux;
- **Multilanguage:** YES (English, Russian);
- **Programming lang.:** [Rust](https://rust-lang.org)
- **GUI:** [iced](https://iced.rs)

<a href="https://iced.rs">
    <img alt="iced" title="iced" src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="350px">
</a>

## License

TimeKeeper is distributed under the [MIT](LICENSE) license.
