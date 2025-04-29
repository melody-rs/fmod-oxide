# fmod-oxide

Safe rust bindings to the FMOD sound engine.
This crate tries to be as rusty and low-cost as possible, without comprimising on any APIs.
Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.

#### Currently in BETA.

I'm currently developing this crate in tandem with another game, which means most of the real world testing of this crate comes from one source and only on a Windows/Linux system.
This means that for some functions/use cases I haven't gotten the API down quite yet. (Custom Filesystems, Mix Matrices, Mac Support, etc)
Almost all of the crate is feature complete though. 
I need to add support for more FMOD versions, and double check the safety of everything before I'm confident releasing this as anything but a beta.

### Using this crate

Due to licensing restrictions this crate can't bundle FMOD, so you'll need to [download](https://www.fmod.com/download) a copy of FMOD yourself.

Make sure to download from `FMOD Engine` specifically.
![Download page screenshot](images/download_page.png?raw=true)

After downloading FMOD, you have to tell this crate where FMOD is located.
**If you're on Windows and used the FMOD installer, you don't have to worry about this.**

The easiest way is to use to create cargo config in your project's root.
`.cargo/config.toml`:
```toml
[env]
FMOD_SYS_FMOD_DIRECTORY = { value = "<absolute install path here>" }
```
You can also specify a relative install path like so:
`.cargo/config.toml`:
```toml
[env]
FMOD_SYS_FMOD_DIRECTORY = { value = "<install path here>", relative = true }
```

Alternatively, you can specify `FMOD_SYS_FMOD_DIRECTORY` when building your project: (not recommended because rust-analyzer won't know this)
`FMOD_SYS_FMOD_DIRECTORY=<path> cargo run`

### Cross compilation

This crate supports cross compilation and will look for a target-specific FMOD install. 

The logic is quite basic at the moment, but it'll look for `<fmod intall dir>/<target os>/<target arch>` first and then look for `<fmod intall dir>/<target os>`.
If no target specific directory was found it'll default to `<fmod install dir>`.

### Using with webassembly

Currently only `wasm32-unknown-emscripten` I know to work, but I can't verify that `wasm32-unknown-unknown` works.
Unfortunately `wasm-bindgen` doesn't work right now, so your milage may vary.

The setup is roughly the same, except you'll need to add some arguments `EMCC_FLAGS`. 
You can do this by editing `.cargo/config.toml`:

```toml
[env]
EMCC_CFLAGS="-s EXPORTED_RUNTIME_METHODS=ccall,cwrap,setValue,getValue" # FMOD requires this
```

If you're using `wasm32-unknown-unknown`, you'll additionally need to add this until [this issue](https://github.com/rust-lang/rust/issues/138762) is closed.
```toml
[build]
rustflags="-Zwasm-c-abi=spec"
```

See [`web-examples/emscripten`](web-examples/emscripten) for a more detailed example.

### Running Examples

Examples expect FMOD to be installed inside `fmod-sys/fmod`. 
You may see an error like `Error: Error { code: FMOD_RESULT(18), message: "File not found." }` if you don't do that!

### Docs

Most documentation is copied directly from the FMOD docs, however some information (such as parameter values) are excluded.
Please refer to the FMOD documentation for more usage information.

# Memory management & Copy types

All FMOD objects are Copy, Clone, Send and Sync because it's possible to have multiple references to the same object. (e.g. loading a bank and then retrieving it by its path)
There are a lot of use-cases where you may want to fetch something (like a bank) and never use it again.
Implementing `Drop` to automatically release things would go against that particular use-case, so this crate opts to have manual `release()` methods instead.

This crate does not currently guard against use-after-frees, *however* using most of FMOD's types (especially FMOD Studio's types) after calling `release()` is safe.
I'm still not 100% sure of what is and isn't safe and I'm actively trying to test this.

# String types
fmod-oxide aims to be as zero-cost as possible, and as such, it uses UTF-8 C strings from the `lanyard` crate as its string type.
This means that all FMOD functions take a `&Utf8CStr` instead of a `&str` or `&CStr`. 
`&Utf8CStr` is pretty cheap to construct (and can even be done statically with the `c!` macro), so this should not be a problem.

When FMOD returns a string, it will always return a `Utf8CString` (the owned version of `Utf8CStr`) because it's difficult to encode lifetime requirements of FMOD strings.

This applies to structs like `fmod::studio::AdvancedSettings` which store C strings. 
Converting structs like `AdvancedSettings` to their FFI equivalents is done by reference as to not pass ownership of the string to FMOD.
FMOD *seems* to copy strings into its own memory, so this is ok?

When converting from an FFI struct to something like `AdvancedSettings`, the C string pointer is copied into a `Utf8CString`. 
*This is unsafe* as there are no guarantees that the pointer is valid or that the string is null-terminated and UTF-8.
Luckily all FMOD functions return UTF-8 strings so this isn't really a problem in practice.

# Undefined Behaviour and unsafe fns

I'm trying to make these bindings as safe as possible, if you find UB please report it!
There are a couple cases related to thread safety (You can initialize FMOD to be thread unsafe) and panics that I am aware of and actively trying to fix.

Right now there are some fns marked as unsafe that I'm not sure how to get working safely. 
System creation, initialization, and cleanup is a pretty big one- creating a system is really unsafe, and certain functions can only be called before or after system creation.
Releasing a system is probably the most unsafe operation of all though, as it invalidates all FMOD handles associated with that system!

# Userdata

Userdata is really, really, really hard to make safe bindings for, because any libary that wants to will need to clean up userdata whenever the FMOD object it is attached to is released.
Unfortunately, there's quite a lot of cases where it's not possible to do that. 
`EventDescription` and `EventInstance` is a pretty good example- you can set a callback when events are released to clean up their userdata. 
You can also set up a callback when banks are unloaded as well to also clean up their userdata. 
That callback would be a perfect place to clean up userdata on `EventDescription` *however* you can't access the `EventDescription`s of a bank when that callback is fired.

I've been thinking on this issue for a few months and I can't find a way to safely do it that doesn't involve significant overhead.
My previous approach involved setting userdata to a slotmap index, and removing things from the slotmap when they were released or stopped being valid.
This was quite jank to say the least and was not a solution I was happy with.

After that, I tried fixing `release()`'s use-after-free issues by storing a `HashSet` of all the pointers FMOD returns from its API, and removing pointers from that `HashSet` when `release()` was called.
This would've had similar issues to the slotmap approach but would drop userdata early and not handle the `EventDescription` issue, so I scrapped it.

Ultimately I've decided to make userdata not the concern of this crate. Setting and fetching it is perfectly safe, *using* the pointer is what's unsafe. 
It's likely better this way- the semantics of userdata are just too flexible and its hard to cover every edge case.
Userdata isn't super commonly used anyway- it is mainly used to pass data into callbacks, but it's easy enough to use a `static` to do that.

If there was an easy way to enforce that a `T` is pointer sized and needs no `Drop` (at compile time) then I could use the approach I was going for early on in this crate and just transmute the `T` to a `*mut c_void`.
(See [this commit](https://github.com/Speak2Erase/fmod-oxide/tree/a14876da32ce5df5b14673c118f09da6fec17544).)

# Differences to other crates
[libfmod](https://github.com/lebedec/libfmod) is similar to this crate, but its major difference is that it is automatically generated from the FMOD documentation instead of using handwritten bindings like this crate.
Because it's automatically generated, it has a much faster release schedule than this crate will, but the API is closer to the C API. If you don't like my crate, it's a pretty decent alternative!

[rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) is outdated, has no studio bindings, and has major safety holes (userdata takes an `&'a mut T` and does no type checking, System creation functions are not marked as unsafe, etc)

[fmod-rs](https://github.com/CAD97/fmod-rs)
I'll be honest, I wasn't aware of this crate until recently. It's missing studio bindings and is designed to be used with bevy. 
There's a couple decisions (like a reference counted Handle type) that are interesting but aren't zero cost.
If my crate doesn't work for you, it's definitely worth checking out!
