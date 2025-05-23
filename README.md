# fmod-oxide

[![Latest version](https://img.shields.io/crates/v/fmod-oxide.svg)](https://crates.io/crates/fmod-oxide)
![Crates.io Total Downloads](https://img.shields.io/crates/d/fmod-oxide)
[![Documentation](https://docs.rs/fmod-oxide/badge.svg)](https://docs.rs/fmod-oxide)
![Crates.io License](https://img.shields.io/crates/l/fmod-oxide)


Safe rust bindings to the FMOD sound engine.
This crate tries to be as rusty and low-cost as possible, without compromising on any APIs.
Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.

---

Supports FMOD versions >2.0.2.28 and >2.0.3.07, and Windows/Linux/MacOS/HTML5 platforms.

Any newer patch-level FMOD versions should compile but might have missing features.

Most of the real world testing and development of this crate comes from one source (me) and only on a Linux system.
Despite that, I'm trying my best to support other platforms. 
If you can't compile/run an example on a specific platform, please file an issue!

#### Currently in BETA.

Almost all of the crate is feature complete!

There's some use cases that I'm still figuring out the exact design for, but they're mainly power user features.
You should be able to use the raw FFI functions from `fmod-audio-sys` even if I don't have a particular API covered.

I need to double check the safety of everything before I'm confident releasing this as anything but a beta.

### Using this crate

Due to licensing restrictions this crate can't bundle FMOD, so you'll need to [download](https://www.fmod.com/download) a copy of FMOD yourself.

Make sure to download from `FMOD Engine` specifically.
![Download page screenshot](https://github.com/melody-rs/fmod-oxide/blob/main/images/download_page.png?raw=true)

After downloading FMOD, you have to tell this crate where FMOD is located.
**If you're on Windows and used the FMOD installer, you don't have to worry about this.**

The easiest way is to create a cargo config in your project's root.

```toml
# `.cargo/config.toml`

[env]
FMOD_SYS_FMOD_DIRECTORY = { value = "<absolute install path here>" }
```

You can also specify a relative install path like so:

```toml
# `.cargo/config.toml`

[env]
FMOD_SYS_FMOD_DIRECTORY = { value = "<install path here>", relative = true }
```

(not recommended because rust-analyzer won't know this) Alternatively, you can specify `FMOD_SYS_FMOD_DIRECTORY` when building your project: 

`FMOD_SYS_FMOD_DIRECTORY=<path> cargo run`

### Cross compilation

This crate supports cross compilation and will look for a target-specific FMOD install. 

The logic is quite basic at the moment, but it'll check if `<fmod install dir>/<target os>` exists and use that.

If no target specific directory was found, it'll default to `<fmod install dir>`

### Using with webassembly

Currently only `wasm32-unknown-emscripten` works well.
`wasm32-unknown-unknown` also works in some capacity but you have to essentially reimplement parts of libc and emscripten. 

Unfortunately `wasm-bindgen` doesn't work without patches right now, so your milage may vary

The setup is roughly the same, except you'll need to add some arguments to `EMCC_FLAGS`.

You can do this by editing `.cargo/config.toml`:
```toml
# `.cargo/config.toml`

[env]
EMCC_CFLAGS="-s EXPORTED_RUNTIME_METHODS=ccall,cwrap,setValue,getValue" # FMOD requires this
```

If you're using `wasm32-unknown-unknown`, you'll additionally need to add this until [this issue](https://github.com/rust-lang/rust/issues/138762) is closed.

```toml
# `.cargo/config.toml`

[build]
rustflags="-Zwasm-c-abi=spec"
```

See [`web-examples/emscripten`](web-examples/emscripten) for a more detailed example.

### Docs

Most documentation is copied directly from the FMOD docs, however some information (such as parameter values) are excluded.
Please refer to the FMOD documentation for more usage information.

### Examples

Examples are pretty much straight ports of their FMOD counterparts, written so you can compare them with the official FMOD examples.
More rusty examples will be provided in the future that showcase how best to use this crate.

# Memory management & Copy types

All FMOD objects are Copy, Clone, Send and Sync because it's possible to have multiple references to the same object. (e.g. loading a bank and then retrieving it by its path)
There are a lot of use-cases where you may want to fetch something (like a bank) and never use it again.
Implementing `Drop` to automatically release things would go against that particular use-case, so this crate opts to have manual `release()` methods instead.

This crate does not currently guard against use-after-frees, *however* using most of FMOD's types (especially FMOD Studio's types) after calling `release()` is safe.
I'm still not 100% sure of what is and isn't safe and I'm actively trying to test this.

# String types

fmod-oxide aims to be as zero-cost as possible, and as such, it uses UTF-8 C strings from the `lanyard` crate as its string type.
This means that all FMOD functions take a `&Utf8CStr` instead of a `&str` or `&CStr`. 
`&Utf8CStr` is pretty cheap to construct (and can even be done statically with the `c!` macro), so this should not be a problem

When FMOD returns a string, it will always return a `Utf8CString` (the owned version of `Utf8CStr`) because it's difficult to encode lifetime requirements of FMOD strings.

This applies to structs like `fmod::studio::AdvancedSettings` which store C strings. 
Converting structs like `AdvancedSettings` to their FFI equivalents is done by reference as to not pass ownership of the string to FMOD

FMOD *seems* to copy strings into its own memory, so this is ok?

# Undefined Behavior and unsafe fns

I'm trying to make these bindings as safe as possible, if you find UB please report it!

Right now there are some fns marked as unsafe that I'm not sure how to get working safely. 
System creation, initialization, and cleanup is a pretty big one- creating a system is really unsafe, and certain functions can only be called before or after system creation.
Releasing a system is probably the most unsafe operation of all though, as it invalidates all FMOD handles associated with that system!

# Userdata

Userdata is really, really, really hard to make safe bindings for, because any library that wants to will need to clean up userdata whenever the FMOD object it is attached to is released.
Unfortunately, there's quite a lot of cases where it's not possible to do that.

`EventDescription` and `EventInstance` is a pretty good example- you can set a callback when events are released to clean up their userdata. 
You can also set up a callback when banks are unloaded as well to also clean up their userdata. 
That callback would be a perfect place to clean up userdata on `EventDescription` *however* you can't access the `EventDescription`s of a bank when that callback is fired.

I've been thinking on this issue for a few months and I can't find a way to safely do it that doesn't involve significant overhead.
Not for a lack of trying- I've tried at least 3 different approaches and couldn't find one I was happy with.

Ultimately I've decided to make userdata not the concern of this crate. Setting and fetching it is perfectly safe, *using* the pointer is what's unsafe. 
It's likely better this way- the semantics of userdata are just too flexible and its hard to cover every edge case.
Userdata isn't super commonly used anyway- it's mainly used to pass data into callbacks, but it's easy enough to use a `static` to do that.

If there was an easy way to enforce that a `T` is pointer sized and needs no `Drop` (at compile time) then I could use the approach I was going for early on in this crate and just transmute the `T` to a `*mut c_void`
(See [this commit](https://github.com/melody-rs/fmod-oxide/tree/a14876da32ce5df5b14673c118f09da6fec17544).)

# Writing DSPs in Rust

Currently not supported. If there's demand for it, I'll add it to the crate though!

# Differences to other crates

[libfmod](https://github.com/lebedec/libfmod) is similar to this crate, but its major difference is that it is automatically generated from the FMOD documentation instead of using handwritten bindings like this crate.
Because it's automatically generated, it has a much faster release schedule than this crate will, but the API is closer to the C API. If you don't like my crate, it's a pretty decent alternative!

[rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) is outdated, has no studio bindings, and has major safety holes (userdata takes an `&'a mut T` and does no type checking, System creation functions are not marked as unsafe, etc)

[fmod-rs](https://github.com/CAD97/fmod-rs)
I'll be honest, I wasn't aware of this crate until recently. It's missing studio bindings and is designed to be used with bevy. 
There's a couple decisions (like a reference counted Handle type) that are interesting but aren't zero cost.
If my crate doesn't work for you, it's definitely worth checking out!
