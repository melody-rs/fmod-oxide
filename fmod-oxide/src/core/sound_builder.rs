use std::ffi::{c_char, c_int, c_uint, c_void};
use std::marker::PhantomData;

use crate::{FmodResultExt, Guid, Result};
use fmod_sys::*;
use lanyard::Utf8CStr;

use crate::{ChannelOrder, Mode, SoundFormat, SoundGroup, SoundType, TimeUnit, panic_wrapper};

use super::{
    FileSystemAsync, FileSystemSync, Sound, System, async_filesystem_cancel, async_filesystem_read,
    filesystem_close, filesystem_open, filesystem_read, filesystem_seek,
};

#[cfg(doc)]
use crate::Error;

/// A builder for creating a [`Sound`].
#[derive(Debug)]
pub struct SoundBuilder<'a> {
    pub(crate) mode: FMOD_MODE,
    pub(crate) create_sound_ex_info: FMOD_CREATESOUNDEXINFO,
    pub(crate) name_or_data: *const c_char,
    pub(crate) _phantom: PhantomData<&'a ()>,
}

const EMPTY_EXINFO: FMOD_CREATESOUNDEXINFO = unsafe {
    FMOD_CREATESOUNDEXINFO {
        cbsize: std::mem::size_of::<FMOD_CREATESOUNDEXINFO>() as c_int,
        ..std::mem::MaybeUninit::zeroed().assume_init()
    }
};

/// Capture or provide sound data as it is decoded.
pub trait PcmCallback {
    /// Callback to provide audio for [`SoundBuilder::open_user`], or capture audio as it is decoded.
    fn read(sound: Sound, data: &mut [u8]) -> Result<()>;

    /// Callback to perform seeking for [`SoundBuilder::open_user`], or capture seek requests.
    fn set_position(
        sound: Sound,
        subsound: c_int,
        position: c_uint,
        position_type: TimeUnit,
    ) -> Result<()>;
}

/// Callback to be called when a sound has finished loading, or a non blocking seek is occuring.
///
/// Return code currently ignored.
///
/// Note that for non blocking streams a seek could occur when restarting the sound after the first playthrough.
/// This will result in a callback being triggered again.
///
/// # Safety
///
/// Since this callback can occur from the async thread, there are restrictions about what functions can be called during the callback.
/// All [`Sound`] functions are safe to call, except for [`Sound::set_sound_group`] and [`Sound::release`].
/// It is also safe to call [`System::get_userdata`].
/// The rest of the Core API and the Studio API is not allowed. Calling a non-allowed function will return [`Error::InvalidThread`].
pub unsafe trait NonBlockCallback {
    /// Call this particular callback.
    // "return code is ignored". so do we want to allow returning a result?
    fn call(sound: Sound, result: Result<()>) -> Result<()>;
}

// setters
impl<'a> SoundBuilder<'a> {
    /// Open a file or url.
    pub const fn open(filename: &'a Utf8CStr) -> Self {
        Self {
            mode: 0,
            create_sound_ex_info: EMPTY_EXINFO,
            name_or_data: filename.as_ptr(),
            _phantom: PhantomData,
        }
    }

    /// Open a user-created static sample or stream.
    pub const fn open_user(
        length: c_uint,
        channel_count: c_int,
        default_frequency: c_int,
        format: SoundFormat,
    ) -> Self {
        Self {
            mode: FMOD_OPENUSER,
            create_sound_ex_info: FMOD_CREATESOUNDEXINFO {
                length,
                numchannels: channel_count,
                defaultfrequency: default_frequency,
                format: format as _,
                ..EMPTY_EXINFO
            },
            name_or_data: std::ptr::null(),
            _phantom: PhantomData,
        }
    }

    /// Open the sound using a byte slice.
    ///
    /// # Safety
    ///
    /// The slice must remain valid until the sound has been *loaded*.
    /// See the [`Mode`] docs for more information.
    pub const unsafe fn open_memory(data: &'a [u8]) -> Self {
        Self {
            mode: FMOD_OPENMEMORY,
            create_sound_ex_info: FMOD_CREATESOUNDEXINFO {
                length: data.len() as c_uint,
                ..EMPTY_EXINFO
            },
            name_or_data: data.as_ptr().cast(),
            _phantom: PhantomData,
        }
    }

    /// Open the sound using a byte slice.
    ///
    /// # Safety
    ///
    /// The slice must remain valid until the sound has been *released*.
    /// Unlike [`Self::open_memory`] this function does not copy the data, so it is even more unsafe!
    pub const unsafe fn open_memory_point(data: &'a [u8]) -> Self {
        Self {
            mode: FMOD_OPENMEMORY_POINT,
            create_sound_ex_info: FMOD_CREATESOUNDEXINFO {
                length: data.len() as c_uint,
                ..EMPTY_EXINFO
            },
            name_or_data: data.as_ptr().cast(),
            _phantom: PhantomData,
        }
    }

    /// Specify a custom filesystem to open the [`Sound`].
    // FIXME is this a valid API?
    #[must_use]
    pub const fn with_filesystem<F: FileSystemSync + FileSystemAsync>(
        mut self,
        userdata: *mut c_void,
    ) -> Self {
        self.create_sound_ex_info.fileuseropen = Some(filesystem_open::<F>);
        self.create_sound_ex_info.fileuserclose = Some(filesystem_close::<F>);
        self.create_sound_ex_info.fileuserread = Some(filesystem_read::<F>);
        self.create_sound_ex_info.fileuserseek = Some(filesystem_seek::<F>);
        self.create_sound_ex_info.fileuserasyncread = Some(async_filesystem_read::<F>);
        self.create_sound_ex_info.fileuserasynccancel = Some(async_filesystem_cancel::<F>);
        self.create_sound_ex_info.fileuserdata = userdata;
        self
    }

    /// Specify a custom *sync* filesystem  to open the [`Sound`].
    #[must_use]
    pub const fn with_filesystem_sync<F: FileSystemSync>(mut self, userdata: *mut c_void) -> Self {
        self.create_sound_ex_info.fileuseropen = Some(filesystem_open::<F>);
        self.create_sound_ex_info.fileuserclose = Some(filesystem_close::<F>);
        self.create_sound_ex_info.fileuserread = Some(filesystem_read::<F>);
        self.create_sound_ex_info.fileuserseek = Some(filesystem_seek::<F>);
        self.create_sound_ex_info.fileuserasyncread = None;
        self.create_sound_ex_info.fileuserasynccancel = None;
        self.create_sound_ex_info.fileuserdata = userdata;
        self
    }

    /// Specify a custom *async* filesystem  to open the [`Sound`].
    #[must_use]
    pub const fn with_filesystem_async<F: FileSystemAsync>(
        mut self,
        userdata: *mut c_void,
    ) -> Self {
        self.create_sound_ex_info.fileuseropen = Some(filesystem_open::<F>);
        self.create_sound_ex_info.fileuserclose = Some(filesystem_close::<F>);
        self.create_sound_ex_info.fileuserasyncread = Some(async_filesystem_read::<F>);
        self.create_sound_ex_info.fileuserasynccancel = Some(async_filesystem_cancel::<F>);
        self.create_sound_ex_info.fileuserread = None;
        self.create_sound_ex_info.fileuserseek = None;
        self.create_sound_ex_info.fileuserdata = userdata;
        self
    }

    /// # Safety
    ///
    /// The [`FMOD_CREATESOUNDEXINFO`] must be valid.
    #[must_use]
    pub const unsafe fn with_raw_ex_info(mut self, ex_info: FMOD_CREATESOUNDEXINFO) -> Self {
        self.create_sound_ex_info = ex_info;
        self
    }

    /// File offset to start reading from.
    #[must_use]
    pub const fn with_file_offset(mut self, file_offset: c_uint) -> Self {
        self.create_sound_ex_info.fileoffset = file_offset;
        self
    }

    /// Ignore the file format and treat as raw PCM.
    #[must_use]
    pub const fn with_open_raw(
        mut self,
        channel_count: c_int,
        default_frequency: c_int,
        format: SoundFormat,
    ) -> Self {
        self.mode |= FMOD_OPENRAW;
        self.create_sound_ex_info.numchannels = channel_count;
        self.create_sound_ex_info.defaultfrequency = default_frequency;
        self.create_sound_ex_info.format = format as _;
        self
    }

    /// Set the [`Mode`] flags for this builder.
    ///
    /// [`Mode::OPEN_MEMORY`], [`Mode::OPEN_MEMORY_POINT`],
    /// [`Mode::OPEN_USER`], and [`Mode::OPEN_RAW`] cannot be set using this function.
    ///
    /// Please use constructors on this type to use those modes.
    #[must_use]
    pub const fn with_mode(mut self, mode: Mode) -> Self {
        const DISABLE_MODES: Mode = Mode::OPEN_MEMORY
            .union(Mode::OPEN_MEMORY_POINT)
            .union(Mode::OPEN_USER)
            .union(Mode::OPEN_RAW);

        let mode = mode.difference(DISABLE_MODES); // these modes are not allowed to be set by the user, so we unset them
        let mode: FMOD_MODE = mode.bits();
        self.mode |= mode;
        self
    }

    /// Size of the decoded buffer for [`Mode::CREATE_STREAM`], or the block size used with pcmreadcallback for [`SoundBuilder::open_user`].
    #[must_use]
    pub const fn with_decode_buffer_size(mut self, size: c_uint) -> Self {
        self.create_sound_ex_info.decodebuffersize = size;
        self
    }

    /// Initial subsound to seek to for [`Mode::CREATE_STREAM`].
    #[must_use]
    pub const fn with_initial_subsound(mut self, initial_subsound: c_int) -> Self {
        self.create_sound_ex_info.initialsubsound = initial_subsound;
        self
    }

    /// Number of subsounds available for [`SoundBuilder::open_user`], or maximum subsounds to load from file.
    #[must_use]
    pub const fn with_subsound_count(mut self, count: c_int) -> Self {
        self.create_sound_ex_info.numsubsounds = count;
        self
    }

    /// List of subsound indices to load from file.
    // TODO: check if this is safe
    #[must_use]
    pub const fn with_inclusion_list(mut self, list: &'a [c_int]) -> Self {
        self.create_sound_ex_info.inclusionlist = list.as_ptr().cast_mut().cast();
        self.create_sound_ex_info.inclusionlistnum = list.len() as c_int;
        self
    }

    /// File path for a [`SoundType::DLS`] sample set to use when loading a [`SoundType::MIDI`] file, see below for defaults.
    // TODO check safety
    #[must_use]
    pub const fn with_dls_name(mut self, dls_name: &'a Utf8CStr) -> Self {
        self.create_sound_ex_info.dlsname = dls_name.as_ptr();
        self
    }

    /// Key for encrypted [`SoundType::FSB`] file, cannot be used in conjunction with [`Self::open_memory_point`].
    // TODO check safety
    #[must_use]
    pub const fn with_encryption_key(mut self, key: &'a Utf8CStr) -> Self {
        self.create_sound_ex_info.encryptionkey = key.as_ptr();
        self
    }

    /// Maximum voice count for [`SoundType::MIDI`] / [`SoundType::IT`].
    #[must_use]
    pub fn with_max_polyphony(mut self, max_polyphony: c_int) -> Self {
        self.create_sound_ex_info.maxpolyphony = max_polyphony;
        self
    }

    /// Attempt to load using the specified type first instead of loading in codec priority order.
    #[must_use]
    pub const fn with_suggested_sound_type(mut self, sound_type: SoundType) -> Self {
        self.create_sound_ex_info.suggestedsoundtype = sound_type as _;
        self
    }

    /// Buffer size for reading the file, -1 to disable buffering.
    #[must_use]
    pub const fn with_file_buffer_size(mut self, size: c_int) -> Self {
        self.create_sound_ex_info.filebuffersize = size;
        self
    }

    /// Custom ordering of speakers for this sound data.
    #[must_use]
    pub const fn with_channel_order(mut self, order: ChannelOrder) -> Self {
        self.create_sound_ex_info.channelorder = order as _;
        self
    }

    /// [`SoundGroup`] to place the created [`Sound`] in once created.
    #[must_use]
    pub fn with_initial_sound_group(mut self, group: SoundGroup) -> Self {
        self.create_sound_ex_info.initialsoundgroup = group.into();
        self
    }

    /// Initial position to seek to for [`Mode::CREATE_STREAM`].
    #[must_use]
    pub const fn with_initial_seek_position(mut self, position: c_uint, unit: TimeUnit) -> Self {
        self.create_sound_ex_info.initialseekposition = position;
        self.create_sound_ex_info.initialseekpostype = unit as _;
        self
    }

    /// Ignore [`System::set_filesystem_sync`] and this [`SoundBuilder`]'s file callbacks.
    #[must_use]
    pub const fn with_ignore_set_filesystem(mut self, ignore: bool) -> Self {
        self.create_sound_ex_info.ignoresetfilesystem = ignore as _;
        self
    }

    /// Hardware / software decoding policy for [`SoundType::AudioQueue`].
    #[must_use]
    pub const fn with_audioqueue_policy(mut self, policy: c_uint) -> Self {
        self.create_sound_ex_info.audioqueuepolicy = policy;
        self
    }

    /// Mixer granularity for [`SoundType::MIDI`] sounds, smaller numbers give a more accurate reproduction at the cost of higher CPU usage.
    #[must_use]
    pub const fn with_min_midi_granularity(mut self, granularity: c_uint) -> Self {
        self.create_sound_ex_info.minmidigranularity = granularity as _;
        self
    }

    /// Thread index to execute [`Mode::NONBLOCKING`] loads on for parallel Sound loading.
    #[must_use]
    pub const fn with_non_block_thread_id(mut self, id: c_int) -> Self {
        self.create_sound_ex_info.nonblockthreadid = id as _;
        self
    }

    /// On input, GUID of already loaded [`SoundType::FSB`] file to reduce disk access, on output, GUID of loaded FSB.
    // TODO check safety
    #[must_use]
    pub const fn with_fsb_guid(mut self, guid: &'a Guid) -> Self {
        self.create_sound_ex_info.fsbguid = std::ptr::from_ref(guid).cast_mut().cast();
        self
    }

    /// Specify a PCM callback.
    #[must_use]
    pub const fn with_pcm_callback<C: PcmCallback>(mut self) -> Self {
        unsafe extern "C" fn pcm_read<C: PcmCallback>(
            sound: *mut FMOD_SOUND,
            data: *mut c_void,
            data_len: c_uint,
        ) -> FMOD_RESULT {
            panic_wrapper(|| {
                let result = C::read(unsafe { Sound::from_ffi(sound) }, unsafe {
                    std::slice::from_raw_parts_mut(data.cast(), data_len as _)
                });
                FMOD_RESULT::from_result(result)
            })
        }
        unsafe extern "C" fn pcm_set_pos<C: PcmCallback>(
            sound: *mut FMOD_SOUND,
            subsound: c_int,
            position: c_uint,
            postype: FMOD_TIMEUNIT,
        ) -> FMOD_RESULT {
            panic_wrapper(|| {
                let result = C::set_position(
                    unsafe { Sound::from_ffi(sound) },
                    subsound,
                    position,
                    postype.try_into().unwrap(),
                );
                FMOD_RESULT::from_result(result)
            })
        }

        self.create_sound_ex_info.pcmreadcallback = Some(pcm_read::<C>);
        self.create_sound_ex_info.pcmsetposcallback = Some(pcm_set_pos::<C>);

        self
    }

    /// Callback to notify completion for [`Mode::NONBLOCKING`], occurs during creation and seeking / restarting streams.
    #[must_use]
    pub const fn with_nonblock_callback<C: NonBlockCallback>(mut self) -> Self {
        unsafe extern "C" fn nonblock_callback<C: NonBlockCallback>(
            sound: *mut FMOD_SOUND,
            result: FMOD_RESULT,
        ) -> FMOD_RESULT {
            panic_wrapper(|| {
                let result = C::call(unsafe { Sound::from_ffi(sound) }, result.to_result());
                FMOD_RESULT::from_result(result)
            })
        }

        self.create_sound_ex_info.nonblockcallback = Some(nonblock_callback::<C>);

        self
    }

    pub(crate) fn ex_info_is_empty(&self) -> bool {
        self.create_sound_ex_info == EMPTY_EXINFO
    }

    /// Helper method that forwards to [`System::create_sound`].
    pub fn build(&self, system: System) -> Result<Sound> {
        system.create_sound(self)
    }

    /// Helper method that forwards to [`System::create_stream`].
    pub fn build_stream(&self, system: System) -> Result<Sound> {
        system.create_stream(self)
    }
}

// getters
impl<'a> SoundBuilder<'a> {
    /// Get the mode of this [`SoundBuilder`].
    pub const fn mode(&self) -> Mode {
        Mode::from_bits_truncate(self.mode)
    }

    /// Get the raw ex info of this [`SoundBuilder`].
    pub const fn raw_ex_info(&self) -> FMOD_CREATESOUNDEXINFO {
        self.create_sound_ex_info
    }

    /// Get the raw name/data/url of this [`SoundBuilder`].
    pub const fn raw_name_or_data(&self) -> *const c_char {
        self.name_or_data
    }

    /// Get the name or url of this [`SoundBuilder`].
    ///
    /// Returns `None` if [`Mode::OPEN_MEMORY`] or [`Mode::OPEN_MEMORY_POINT`] or [`Mode::OPEN_USER`] are set.
    pub fn name_or_url(&self) -> Option<&Utf8CStr> {
        if self
            .mode()
            .intersects(Mode::OPEN_MEMORY | Mode::OPEN_MEMORY_POINT | Mode::OPEN_USER)
        {
            None
        } else {
            Some(unsafe { Utf8CStr::from_ptr_unchecked(self.name_or_data) })
        }
    }

    /// Get the data of this [`SoundBuilder`].
    ///
    /// Returns `Some` if [`Mode::OPEN_MEMORY`] or [`Mode::OPEN_MEMORY_POINT`] are set.
    pub fn data(&self) -> Option<&[u8]> {
        if self
            .mode()
            .intersects(Mode::OPEN_MEMORY | Mode::OPEN_MEMORY_POINT)
        {
            Some(unsafe {
                std::slice::from_raw_parts(
                    self.name_or_data.cast(),
                    self.create_sound_ex_info.length as usize,
                )
            })
        } else {
            None
        }
    }

    /// Get the length of data of this [`SoundBuilder`].
    pub const fn length(&self) -> c_uint {
        self.create_sound_ex_info.length
    }

    /// Get the file offset of this [`SoundBuilder`].
    pub const fn file_offset(&self) -> c_uint {
        self.create_sound_ex_info.fileoffset
    }

    /// Get the channel count of this [`SoundBuilder`].
    pub const fn channel_count(&self) -> c_int {
        self.create_sound_ex_info.numchannels
    }

    /// Get the default frequency of this [`SoundBuilder`].
    pub const fn default_frequency(&self) -> c_int {
        self.create_sound_ex_info.defaultfrequency
    }

    /// Get the sound format of this [`SoundBuilder`].
    #[allow(clippy::missing_panics_doc)] // this function can't panic in practice as we control the sound format
    pub fn format(&self) -> SoundFormat {
        self.create_sound_ex_info.format.try_into().unwrap()
    }

    /// Get the decode buffer size of this [`SoundBuilder`].
    pub const fn decode_buffer_size(&self) -> c_uint {
        self.create_sound_ex_info.decodebuffersize
    }

    /// Get the initial subsound of this [`SoundBuilder`].
    pub const fn initial_subsound(&self) -> c_int {
        self.create_sound_ex_info.initialsubsound
    }

    /// Get the subsound count of this [`SoundBuilder`].
    pub const fn subsound_count(&self) -> c_int {
        self.create_sound_ex_info.numsubsounds
    }

    /// Get the inclusion list of this [`SoundBuilder`].
    pub fn inclusion_list(&self) -> Option<&'a [c_int]> {
        if self.create_sound_ex_info.inclusionlist.is_null() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(
                    self.create_sound_ex_info.inclusionlist.cast(),
                    self.create_sound_ex_info.inclusionlistnum as usize,
                )
            })
        }
    }

    /// Get the DLS name of this [`SoundBuilder`].
    pub fn dls_name(&self) -> Option<&Utf8CStr> {
        if self.create_sound_ex_info.dlsname.is_null() {
            None
        } else {
            Some(unsafe { Utf8CStr::from_ptr_unchecked(self.create_sound_ex_info.dlsname) })
        }
    }

    /// Get the encryption key of this [`SoundBuilder`].
    pub fn encryption_key(&self) -> Option<&Utf8CStr> {
        if self.create_sound_ex_info.encryptionkey.is_null() {
            None
        } else {
            Some(unsafe { Utf8CStr::from_ptr_unchecked(self.create_sound_ex_info.encryptionkey) })
        }
    }

    /// Get the max polyphony of this [`SoundBuilder`].
    pub const fn max_polyphony(&self) -> c_int {
        self.create_sound_ex_info.maxpolyphony
    }

    /// Get the suggested sound type of this [`SoundBuilder`].
    #[allow(clippy::missing_panics_doc)] // this function can't panic in practice as we control the sound type
    pub fn suggested_sound_type(&self) -> SoundType {
        self.create_sound_ex_info
            .suggestedsoundtype
            .try_into()
            .unwrap()
    }

    /// Get the file buffer size of this [`SoundBuilder`].
    pub const fn file_buffer_size(&self) -> c_int {
        self.create_sound_ex_info.filebuffersize
    }

    /// Get the channel order of this [`SoundBuilder`].
    #[allow(clippy::missing_panics_doc)] // this function can't panic in practice as we control the channel order
    pub fn channel_order(&self) -> ChannelOrder {
        self.create_sound_ex_info.channelorder.try_into().unwrap()
    }

    /// Get the initial sound group of this [`SoundBuilder`].
    pub fn initial_sound_group(&self) -> Option<SoundGroup> {
        if self.create_sound_ex_info.initialsoundgroup.is_null() {
            None
        } else {
            Some(unsafe { SoundGroup::from_ffi(self.create_sound_ex_info.initialsoundgroup) })
        }
    }

    /// Get the initial seek position of this [`SoundBuilder`].
    #[allow(clippy::missing_panics_doc)] // this function can't panic in practice as we control the seek position
    pub fn initial_seek_position(&self) -> (c_uint, TimeUnit) {
        (
            self.create_sound_ex_info.initialseekposition,
            self.create_sound_ex_info
                .initialseekpostype
                .try_into()
                .unwrap(),
        )
    }

    /// Get the ignore set filesystem flag of this [`SoundBuilder`].
    pub const fn ignore_set_filesystem(&self) -> bool {
        self.create_sound_ex_info.ignoresetfilesystem > 0
    }
    /// Get the min midi granularity of this [`SoundBuilder`].
    pub const fn min_midi_granularity(&self) -> c_uint {
        self.create_sound_ex_info.minmidigranularity
    }

    /// Get the nonblock thread id of this [`SoundBuilder`].
    pub const fn non_block_thread_id(&self) -> c_int {
        self.create_sound_ex_info.nonblockthreadid
    }

    /// Get the FSB guid of this [`SoundBuilder`].
    pub const fn fsb_guid(&self) -> Option<Guid> {
        if self.create_sound_ex_info.fsbguid.is_null() {
            None
        } else {
            Some(unsafe { *(self.create_sound_ex_info.fsbguid.cast()) })
        }
    }
}

impl SoundBuilder<'_> {
    /// # Safety
    ///
    /// The mode must match the required fields of the [`FMOD_CREATESOUNDEXINFO`] struct.
    /// The [`FMOD_CREATESOUNDEXINFO`] struct's cbsize field must be set to the size of the struct.
    ///
    /// If the mode is not [`Mode::OPEN_MEMORY`] or [`Mode::OPEN_MEMORY_POINT`] `name_or_data` pointer must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// If the mode is [`Mode::OPEN_MEMORY`] or [`Mode::OPEN_MEMORY_POINT`] the data pointer must be valid for reads of bytes up to [`FMOD_CREATESOUNDEXINFO::length`].
    ///
    /// The lifetime of the builder is unbounded and MUST be constrained!
    pub unsafe fn from_ffi(
        name_or_data: *const c_char,
        mode: FMOD_MODE,
        create_sound_ex_info: FMOD_CREATESOUNDEXINFO,
    ) -> Self {
        Self {
            mode,
            create_sound_ex_info,
            name_or_data,
            _phantom: PhantomData,
        }
    }
}
