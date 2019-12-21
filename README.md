# scthing

A small, flexible embedded Linux device for running and manipulating SuperCollider patches.  Load and unload synths you define, adjust synth parameters live and configure a visual menu system that fits your workflow.  Your custom menu hierarchy and synth parameters are defined in a [config file](example/config.toml) and used to send the appropriate OSC messages to a running SuperCollider server (typically on the same device).

## Hardware

* Linux SoC (i.e. a Raspberry Pi)
* some sort of screen accessible via a framebuffer (I use an SSD1306 128x64 OLED with the [fbtft](https://github.com/notro/fbtft) driver)
* rotary encoder
* button (ideally combined w/ the rotary encoder)
* audio codec (I'm using a WM8731)

**Raspberry Pi setup:**

To configure the rotary encoder and button, add the following to `/boot/config.txt`:

```
dtoverlay=rotary-encoder,pin_a=17,pin_b=27,relative_axis=1
dtoverlay=gpio-key,gpio=22,keycode=28,label="ENTER"
```

To configure an `fbtft` screen, add the following to `/etc/rc.local`:

```
modprobe fbtft_device name=adafruit13m debug=1 speed=2000000 gpios=reset:24,dc:23
```

And `dtparam=spi=on` to `/boot/config.txt`.

(note that there's nothing special about these pin numbers, any GPIO should be fine)

Installing SuperCollider: [https://supercollider.github.io/development/building-raspberrypi](https://supercollider.github.io/development/building-raspberrypi)

## Install

Currently requires the Rust toolchain.

```bash
rustup target add <target>
cargo build --target=<target> --release
```

For Raspberry Pi, the target will be `arm-unknown-linux-gnueabi` (Zero) or `armv7-unknown-linux-gnueabi` (3b+).

Copy `target/<target>/release/scthing` to the target system.

## Usage

```
./scthing -c example/config.toml`
```

## Config

See [example/config.toml](example/config.toml) for an example config file.  The `[devices]` and `[osc]` sections are required but the rest is up to you.

## OSC Protocol

Load a synth:

```
/start <name>
```

Unload a synth:

```
/stop <name>
```

Set a synth parameter:

```
/set <synth name> <arg name> <arg value>
```

Your SC patch should respond to these messages accordingly (see [example/patch.scd](example/patch.scd) for an example).
