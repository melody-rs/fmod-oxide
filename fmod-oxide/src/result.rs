use fmod_sys::*;

#[derive(Clone, PartialEq, Eq, Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error(
        "Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound=>=>lock on a streaming sound)."
    )]
    BadCommand,
    #[error("Error trying to allocate a channel.")]
    ChannelAlloc,
    #[error("The specified channel has been reused to play another sound.")]
    ChannelStolen,
    #[error("DMA Failure.  See debug output for more information.")]
    DMA,
    #[error(
        "DSP connection error.  Connection possibly caused a cyclic dependency or connected dsps with incompatible buffer counts."
    )]
    DspConnection,
    #[error(
        "DSP  code from a DSP process query callback.  Tells mixer not to call the process callback and therefore not consume CPU.  Use this to optimize the DSP graph."
    )]
    DspDontProcess,
    #[error(
        "DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format, or a matrix may have been set with the wrong size if the target unit has a specified channel map."
    )]
    DspFormat,
    #[error(
        "DSP is already in the mixer's DSP network. It must be removed before being reinserted or released."
    )]
    DspInuse,
    #[error("DSP connection error.  Couldn't find the DSP unit specified.")]
    DspNotFound,
    #[error(
        "DSP operation error.  Cannot perform operation on this DSP as it is reserved by the system."
    )]
    DspReserved,
    #[error(
        "DSP operation error.  Cannot perform operation on this DSP as it is reserved by the system."
    )]
    DspSilence,
    #[error("DSP operation cannot be performed on a DSP of this type.")]
    DspType,
    #[error("Error loading file.")]
    FileBad,
    #[error(
        "Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format."
    )]
    FileCouldNotSeek,
    #[error("Media was ejected while reading.")]
    FileDiskEjected,
    #[error("End of file unexpectedly reached while trying to read essential data (truncated?).")]
    FileEof,
    #[error("End of current chunk reached while trying to read data.")]
    FileEndOfData,
    #[error("File not found.")]
    FileNotFound,
    #[error("Unsupported file or audio format.")]
    Format,
    #[error(
        "There is a version mismatch between the FMOD header and either the FMOD Studio library or the FMOD Low Level library."
    )]
    HeaderMismatch,
    #[error("A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere.")]
    Http,
    #[error("The specified resource requires authentication or is forbidden.")]
    HttpAccess,
    #[error("Proxy authentication is required to access the specified resource.")]
    HttpProxyAuth,
    #[error("A HTTP server error occurred.")]
    HttpServerError,
    #[error("The HTTP request timed out.")]
    HttpTimeout,
    #[error("FMOD was not initialized correctly to support this function.")]
    Initialization,
    #[error("Cannot call this command after System::init.")]
    Initialized,
    #[error(
        "An error occured in the FMOD system. Use the logging version of FMOD for more information."
    )]
    Internal,
    #[error("Value passed in was a NaN, Inf or denormalized float.")]
    InvalidFloat,
    #[error("An invalid object handle was used.")]
    InvalidHandle,
    #[error("An invalid parameter was passed to this function.")]
    InvalidParam,
    #[error("An invalid seek position was passed to this function.")]
    InvalidPosition,
    #[error("An invalid speaker was passed to this function based on the current speaker mode.")]
    InvalidSpeaker,
    #[error("The syncpoint did not come from this sound handle.")]
    InvalidSyncPoint,
    #[error("Tried to call a function on a thread that is not supported.")]
    InvalidThread,
    #[error("The vectors passed in are not unit length, or perpendicular.")]
    InvalidVector,
    #[error("Reached maximum audible playback count for this sound's soundgroup.")]
    MaxAudible,
    #[error("Not enough memory or resources.")]
    Memory,
    #[error(
        "Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used."
    )]
    MemoryCantPoint,
    #[error("Tried to call a command on a 2d sound when the command was meant for 3d sound.")]
    Needs3D,
    #[error("Tried to use a feature that requires hardware support.")]
    NeedsHardWare,
    #[error("Couldn't connect to the specified host.")]
    NetConnect,
    #[error(
        "A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere."
    )]
    NetSocketError,
    #[error("The specified URL couldn't be resolved.")]
    NetUrl,
    #[error("The specified URL couldn't be resolved.")]
    NetWouldBlock,
    #[error(
        "Operation could not be performed because specified sound/DSP connection is not ready."
    )]
    NotReady,
    #[error(
        "Error initializing output device, but more specifically, the output device is already in use and cannot be reused."
    )]
    OutputAllocated,
    #[error("Error creating hardware sound buffer.")]
    OutputCreateBuffer,
    #[error(
        "A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted."
    )]
    OuputDriverCall,
    #[error("Soundcard does not support the specified format.")]
    OutputFormat,
    #[error("Error initializing output device.")]
    OutputInit,
    #[error(
        "The output device has no drivers installed.  If pre-init, FMOD_OUTPUT_NOSOUND is selected as the output mode.  If post-init, the function just fails."
    )]
    OutputNoDrivers,
    #[error("An unspecified error has been returned from a plugin.")]
    Plugin,
    #[error("A requested output, dsp unit type or codec was not available.")]
    PluginMissing,
    #[error(
        "A resource that the plugin requires cannot be allocated or found. (ie the DLS file for MIDI playback)"
    )]
    PluginResource,
    #[error("A plugin was built with an unsupported SDK version.")]
    PluginVersion,
    #[error("An error occurred trying to initialize the recording device.")]
    Record,
    #[error(
        "Reverb properties cannot be set on this channel because a parent channelgroup owns the reverb connection."
    )]
    ReverbChannelGroup,
    #[error(
        "Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesn't exist."
    )]
    ReverbInstance,
    #[error(
        "The error occurred because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound."
    )]
    Subsounds,
    #[error(
        "This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first."
    )]
    SubsoundAllocated,
    #[error(
        "Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file."
    )]
    SubsoundCantMove,
    #[error("The specified tag could not be found or there are no tags.")]
    TagNotFound,
    #[error(
        "The sound created exceeds the allowable input channel count.  This can be increased using the 'maxinputchannels' parameter in System::setSoftwareFormat."
    )]
    TooManyChannels,
    #[error(
        "The retrieved string is too long to fit in the supplied buffer and has been truncated."
    )]
    Truncated,
    #[error("Something in FMOD hasn't been implemented when it should be. Contact support.")]
    Unimplemented,
    #[error("This command failed because System::init or System::setDriver was not called.")]
    Uninitialized,
    #[error(
        "A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified."
    )]
    Unsupported,
    #[error("The version number of this file format is not supported.")]
    Version,
    #[error("The specified bank has already been loaded.")]
    EventAlreadyLoaded,
    #[error("The live update connection failed due to the game already being connected.")]
    EventLiveUpdateBusy,
    #[error(
        "The live update connection failed due to the game data being out of sync with the tool."
    )]
    EventLiveUpdateMismatch,
    #[error("The live update connection timed out.")]
    EventLiveUpdateTimeout,
    #[error("The requested event, parameter, bus or vca could not be found.")]
    EventNotFound,
    #[error("The Studio::System object is not yet initialized.")]
    StudioUninitialized,
    #[error("The specified resource is not loaded, so it can't be unloaded.")]
    StudioNotLoaded,
    #[error("An invalid string was passed to this function.")]
    InvalidString,
    #[error("The specified resource is already locked.")]
    AlreadyLocked,
    #[error("The specified resource is not locked, so it can't be unlocked.")]
    NotLocked,
    #[error("The specified recording driver has been disconnected.")]
    RecordDisconnected,
    #[error("The length provided exceeds the allowable limit.")]
    TooManySamples,

    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),
    #[error("No discriminant in enum `{name}` matches the value `{primitive:?}")]
    EnumFromPrivitive { name: &'static str, primitive: i64 },
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<FMOD_RESULT> for Error {
    fn from(value: FMOD_RESULT) -> Self {
        match value {
            FMOD_RESULT::FMOD_ERR_BADCOMMAND => Error::BadCommand,
            FMOD_RESULT::FMOD_ERR_CHANNEL_ALLOC => Error::ChannelAlloc,
            FMOD_RESULT::FMOD_ERR_CHANNEL_STOLEN => Error::ChannelStolen,
            FMOD_RESULT::FMOD_ERR_DMA => Error::DMA,
            FMOD_RESULT::FMOD_ERR_DSP_CONNECTION => Error::DspConnection,
            FMOD_RESULT::FMOD_ERR_DSP_DONTPROCESS => Error::DspDontProcess,
            FMOD_RESULT::FMOD_ERR_DSP_FORMAT => Error::DspFormat,
            FMOD_RESULT::FMOD_ERR_DSP_INUSE => Error::DspInuse,
            FMOD_RESULT::FMOD_ERR_DSP_NOTFOUND => Error::DspNotFound,
            FMOD_RESULT::FMOD_ERR_DSP_RESERVED => Error::DspReserved,
            FMOD_RESULT::FMOD_ERR_DSP_SILENCE => Error::DspSilence,
            FMOD_RESULT::FMOD_ERR_DSP_TYPE => Error::DspType,
            FMOD_RESULT::FMOD_ERR_FILE_BAD => Error::FileBad,
            FMOD_RESULT::FMOD_ERR_FILE_COULDNOTSEEK => Error::FileCouldNotSeek,
            FMOD_RESULT::FMOD_ERR_FILE_DISKEJECTED => Error::FileDiskEjected,
            FMOD_RESULT::FMOD_ERR_FILE_EOF => Error::FileEof,
            FMOD_RESULT::FMOD_ERR_FILE_ENDOFDATA => Error::FileEndOfData,
            FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND => Error::FileNotFound,
            FMOD_RESULT::FMOD_ERR_FORMAT => Error::Format,
            FMOD_RESULT::FMOD_ERR_HEADER_MISMATCH => Error::HeaderMismatch,
            FMOD_RESULT::FMOD_ERR_HTTP => Error::Http,
            FMOD_RESULT::FMOD_ERR_HTTP_ACCESS => Error::HttpAccess,
            FMOD_RESULT::FMOD_ERR_HTTP_PROXY_AUTH => Error::HttpProxyAuth,
            FMOD_RESULT::FMOD_ERR_HTTP_SERVER_ERROR => Error::HttpServerError,
            FMOD_RESULT::FMOD_ERR_HTTP_TIMEOUT => Error::HttpTimeout,
            FMOD_RESULT::FMOD_ERR_INITIALIZATION => Error::Initialization,
            FMOD_RESULT::FMOD_ERR_INITIALIZED => Error::Initialized,
            FMOD_RESULT::FMOD_ERR_INTERNAL => Error::Internal,
            FMOD_RESULT::FMOD_ERR_INVALID_FLOAT => Error::InvalidFloat,
            FMOD_RESULT::FMOD_ERR_INVALID_HANDLE => Error::InvalidHandle,
            FMOD_RESULT::FMOD_ERR_INVALID_PARAM => Error::InvalidParam,
            FMOD_RESULT::FMOD_ERR_INVALID_POSITION => Error::InvalidPosition,
            FMOD_RESULT::FMOD_ERR_INVALID_SPEAKER => Error::InvalidSpeaker,
            FMOD_RESULT::FMOD_ERR_INVALID_SYNCPOINT => Error::InvalidSyncPoint,
            FMOD_RESULT::FMOD_ERR_INVALID_THREAD => Error::InvalidThread,
            FMOD_RESULT::FMOD_ERR_INVALID_VECTOR => Error::InvalidVector,
            FMOD_RESULT::FMOD_ERR_MAXAUDIBLE => Error::MaxAudible,
            FMOD_RESULT::FMOD_ERR_MEMORY => Error::Memory,
            FMOD_RESULT::FMOD_ERR_MEMORY_CANTPOINT => Error::MemoryCantPoint,
            FMOD_RESULT::FMOD_ERR_NEEDS3D => Error::Needs3D,
            FMOD_RESULT::FMOD_ERR_NEEDSHARDWARE => Error::NeedsHardWare,
            FMOD_RESULT::FMOD_ERR_NET_CONNECT => Error::NetConnect,
            FMOD_RESULT::FMOD_ERR_NET_SOCKET_ERROR => Error::NetSocketError,
            FMOD_RESULT::FMOD_ERR_NET_URL => Error::NetUrl,
            FMOD_RESULT::FMOD_ERR_NET_WOULD_BLOCK => Error::NetWouldBlock,
            FMOD_RESULT::FMOD_ERR_NOTREADY => Error::NotReady,
            FMOD_RESULT::FMOD_ERR_OUTPUT_ALLOCATED => Error::OutputAllocated,
            FMOD_RESULT::FMOD_ERR_OUTPUT_CREATEBUFFER => Error::OutputCreateBuffer,
            FMOD_RESULT::FMOD_ERR_OUTPUT_DRIVERCALL => Error::OuputDriverCall,
            FMOD_RESULT::FMOD_ERR_OUTPUT_FORMAT => Error::OutputFormat,
            FMOD_RESULT::FMOD_ERR_OUTPUT_INIT => Error::OutputInit,
            FMOD_RESULT::FMOD_ERR_OUTPUT_NODRIVERS => Error::OutputNoDrivers,
            FMOD_RESULT::FMOD_ERR_PLUGIN => Error::Plugin,
            FMOD_RESULT::FMOD_ERR_PLUGIN_MISSING => Error::PluginMissing,
            FMOD_RESULT::FMOD_ERR_PLUGIN_RESOURCE => Error::PluginResource,
            FMOD_RESULT::FMOD_ERR_PLUGIN_VERSION => Error::PluginVersion,
            FMOD_RESULT::FMOD_ERR_RECORD => Error::Record,
            FMOD_RESULT::FMOD_ERR_REVERB_CHANNELGROUP => Error::ReverbChannelGroup,
            FMOD_RESULT::FMOD_ERR_REVERB_INSTANCE => Error::ReverbInstance,
            FMOD_RESULT::FMOD_ERR_SUBSOUNDS => Error::Subsounds,
            FMOD_RESULT::FMOD_ERR_SUBSOUND_ALLOCATED => Error::SubsoundAllocated,
            FMOD_RESULT::FMOD_ERR_SUBSOUND_CANTMOVE => Error::SubsoundCantMove,
            FMOD_RESULT::FMOD_ERR_TAGNOTFOUND => Error::TagNotFound,
            FMOD_RESULT::FMOD_ERR_TOOMANYCHANNELS => Error::TooManyChannels,
            FMOD_RESULT::FMOD_ERR_TRUNCATED => Error::Truncated,
            FMOD_RESULT::FMOD_ERR_UNIMPLEMENTED => Error::Unimplemented,
            FMOD_RESULT::FMOD_ERR_UNINITIALIZED => Error::Uninitialized,
            FMOD_RESULT::FMOD_ERR_UNSUPPORTED => Error::Unsupported,
            FMOD_RESULT::FMOD_ERR_VERSION => Error::Version,
            FMOD_RESULT::FMOD_ERR_EVENT_ALREADY_LOADED => Error::EventAlreadyLoaded,
            FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_BUSY => Error::EventLiveUpdateBusy,
            FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH => Error::EventLiveUpdateMismatch,
            FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT => Error::EventLiveUpdateTimeout,
            FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND => Error::EventNotFound,
            FMOD_RESULT::FMOD_ERR_STUDIO_UNINITIALIZED => Error::StudioUninitialized,
            FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED => Error::StudioNotLoaded,
            FMOD_RESULT::FMOD_ERR_INVALID_STRING => Error::InvalidString,
            FMOD_RESULT::FMOD_ERR_ALREADY_LOCKED => Error::AlreadyLocked,
            FMOD_RESULT::FMOD_ERR_NOT_LOCKED => Error::NotLocked,
            FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED => Error::RecordDisconnected,
            FMOD_RESULT::FMOD_ERR_TOOMANYSAMPLES => Error::TooManySamples,
            _ => panic!("invalid value"),
        }
    }
}

impl<T> From<num_enum::TryFromPrimitiveError<T>> for Error
where
    T: num_enum::TryFromPrimitive,
    T::Primitive: Into<i64>,
{
    fn from(value: num_enum::TryFromPrimitiveError<T>) -> Self {
        Self::EnumFromPrivitive {
            name: T::NAME,
            primitive: value.number.into(),
        }
    }
}

impl From<Error> for FMOD_RESULT {
    fn from(val: Error) -> Self {
        match val {
            Error::BadCommand => FMOD_RESULT::FMOD_ERR_BADCOMMAND,
            Error::ChannelAlloc => FMOD_RESULT::FMOD_ERR_CHANNEL_ALLOC,
            Error::ChannelStolen => FMOD_RESULT::FMOD_ERR_CHANNEL_STOLEN,
            Error::DMA => FMOD_RESULT::FMOD_ERR_DMA,
            Error::DspConnection => FMOD_RESULT::FMOD_ERR_DSP_CONNECTION,
            Error::DspDontProcess => FMOD_RESULT::FMOD_ERR_DSP_DONTPROCESS,
            Error::DspFormat => FMOD_RESULT::FMOD_ERR_DSP_FORMAT,
            Error::DspInuse => FMOD_RESULT::FMOD_ERR_DSP_INUSE,
            Error::DspNotFound => FMOD_RESULT::FMOD_ERR_DSP_NOTFOUND,
            Error::DspReserved => FMOD_RESULT::FMOD_ERR_DSP_RESERVED,
            Error::DspSilence => FMOD_RESULT::FMOD_ERR_DSP_SILENCE,
            Error::DspType => FMOD_RESULT::FMOD_ERR_DSP_TYPE,
            Error::FileBad => FMOD_RESULT::FMOD_ERR_FILE_BAD,
            Error::FileCouldNotSeek => FMOD_RESULT::FMOD_ERR_FILE_COULDNOTSEEK,
            Error::FileDiskEjected => FMOD_RESULT::FMOD_ERR_FILE_DISKEJECTED,
            Error::FileEof => FMOD_RESULT::FMOD_ERR_FILE_EOF,
            Error::FileEndOfData => FMOD_RESULT::FMOD_ERR_FILE_ENDOFDATA,
            Error::FileNotFound => FMOD_RESULT::FMOD_ERR_FILE_NOTFOUND,
            Error::Format => FMOD_RESULT::FMOD_ERR_FORMAT,
            Error::HeaderMismatch => FMOD_RESULT::FMOD_ERR_HEADER_MISMATCH,
            Error::Http => FMOD_RESULT::FMOD_ERR_HTTP,
            Error::HttpAccess => FMOD_RESULT::FMOD_ERR_HTTP_ACCESS,
            Error::HttpProxyAuth => FMOD_RESULT::FMOD_ERR_HTTP_PROXY_AUTH,
            Error::HttpServerError => FMOD_RESULT::FMOD_ERR_HTTP_SERVER_ERROR,
            Error::HttpTimeout => FMOD_RESULT::FMOD_ERR_HTTP_TIMEOUT,
            Error::Initialization => FMOD_RESULT::FMOD_ERR_INITIALIZATION,
            Error::Initialized => FMOD_RESULT::FMOD_ERR_INITIALIZED,
            Error::Internal => FMOD_RESULT::FMOD_ERR_INTERNAL,
            Error::InvalidFloat => FMOD_RESULT::FMOD_ERR_INVALID_FLOAT,
            Error::InvalidHandle => FMOD_RESULT::FMOD_ERR_INVALID_HANDLE,
            Error::InvalidParam => FMOD_RESULT::FMOD_ERR_INVALID_PARAM,
            Error::InvalidPosition => FMOD_RESULT::FMOD_ERR_INVALID_POSITION,
            Error::InvalidSpeaker => FMOD_RESULT::FMOD_ERR_INVALID_SPEAKER,
            Error::InvalidSyncPoint => FMOD_RESULT::FMOD_ERR_INVALID_SYNCPOINT,
            Error::InvalidThread => FMOD_RESULT::FMOD_ERR_INVALID_THREAD,
            Error::InvalidVector => FMOD_RESULT::FMOD_ERR_INVALID_VECTOR,
            Error::MaxAudible => FMOD_RESULT::FMOD_ERR_MAXAUDIBLE,
            Error::Memory => FMOD_RESULT::FMOD_ERR_MEMORY,
            Error::MemoryCantPoint => FMOD_RESULT::FMOD_ERR_MEMORY_CANTPOINT,
            Error::Needs3D => FMOD_RESULT::FMOD_ERR_NEEDS3D,
            Error::NeedsHardWare => FMOD_RESULT::FMOD_ERR_NEEDSHARDWARE,
            Error::NetConnect => FMOD_RESULT::FMOD_ERR_NET_CONNECT,
            Error::NetSocketError => FMOD_RESULT::FMOD_ERR_NET_SOCKET_ERROR,
            Error::NetUrl => FMOD_RESULT::FMOD_ERR_NET_URL,
            Error::NetWouldBlock => FMOD_RESULT::FMOD_ERR_NET_WOULD_BLOCK,
            Error::NotReady => FMOD_RESULT::FMOD_ERR_NOTREADY,
            Error::OutputAllocated => FMOD_RESULT::FMOD_ERR_OUTPUT_ALLOCATED,
            Error::OutputCreateBuffer => FMOD_RESULT::FMOD_ERR_OUTPUT_CREATEBUFFER,
            Error::OuputDriverCall => FMOD_RESULT::FMOD_ERR_OUTPUT_DRIVERCALL,
            Error::OutputFormat => FMOD_RESULT::FMOD_ERR_OUTPUT_FORMAT,
            Error::OutputInit => FMOD_RESULT::FMOD_ERR_OUTPUT_INIT,
            Error::OutputNoDrivers => FMOD_RESULT::FMOD_ERR_OUTPUT_NODRIVERS,
            Error::Plugin => FMOD_RESULT::FMOD_ERR_PLUGIN,
            Error::PluginMissing => FMOD_RESULT::FMOD_ERR_PLUGIN_MISSING,
            Error::PluginResource => FMOD_RESULT::FMOD_ERR_PLUGIN_RESOURCE,
            Error::PluginVersion => FMOD_RESULT::FMOD_ERR_PLUGIN_VERSION,
            Error::Record => FMOD_RESULT::FMOD_ERR_RECORD,
            Error::ReverbChannelGroup => FMOD_RESULT::FMOD_ERR_REVERB_CHANNELGROUP,
            Error::ReverbInstance => FMOD_RESULT::FMOD_ERR_REVERB_INSTANCE,
            Error::Subsounds => FMOD_RESULT::FMOD_ERR_SUBSOUNDS,
            Error::SubsoundAllocated => FMOD_RESULT::FMOD_ERR_SUBSOUND_ALLOCATED,
            Error::SubsoundCantMove => FMOD_RESULT::FMOD_ERR_SUBSOUND_CANTMOVE,
            Error::TagNotFound => FMOD_RESULT::FMOD_ERR_TAGNOTFOUND,
            Error::TooManyChannels => FMOD_RESULT::FMOD_ERR_TOOMANYCHANNELS,
            Error::Truncated => FMOD_RESULT::FMOD_ERR_TRUNCATED,
            Error::Unimplemented => FMOD_RESULT::FMOD_ERR_UNIMPLEMENTED,
            Error::Uninitialized => FMOD_RESULT::FMOD_ERR_UNINITIALIZED,
            Error::Unsupported => FMOD_RESULT::FMOD_ERR_UNSUPPORTED,
            Error::Version => FMOD_RESULT::FMOD_ERR_VERSION,
            Error::EventAlreadyLoaded => FMOD_RESULT::FMOD_ERR_EVENT_ALREADY_LOADED,
            Error::EventLiveUpdateBusy => FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_BUSY,
            Error::EventLiveUpdateMismatch => FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH,
            Error::EventLiveUpdateTimeout => FMOD_RESULT::FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT,
            Error::EventNotFound => FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND,
            Error::StudioUninitialized => FMOD_RESULT::FMOD_ERR_STUDIO_UNINITIALIZED,
            Error::StudioNotLoaded => FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED,
            Error::InvalidString => FMOD_RESULT::FMOD_ERR_INVALID_STRING,
            Error::AlreadyLocked => FMOD_RESULT::FMOD_ERR_ALREADY_LOCKED,
            Error::NotLocked => FMOD_RESULT::FMOD_ERR_NOT_LOCKED,
            Error::RecordDisconnected => FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED,
            Error::TooManySamples => FMOD_RESULT::FMOD_ERR_TOOMANYSAMPLES,
            Error::NulError(_) | Error::EnumFromPrivitive { .. } => {
                FMOD_RESULT::FMOD_ERR_INVALID_PARAM
            }
        }
    }
}

pub(crate) trait FmodResultExt {
    fn to_result(self) -> Result<()>;

    fn to_error(self) -> Option<Error>;

    fn from_result<T>(result: Result<T>) -> Self;
}

impl FmodResultExt for FMOD_RESULT {
    fn to_result(self) -> Result<()> {
        if matches!(self, FMOD_RESULT::FMOD_OK) {
            Ok(())
        } else {
            Err(self.into())
        }
    }

    fn to_error(self) -> Option<Error> {
        self.to_result().err()
    }

    fn from_result<T>(result: Result<T>) -> Self {
        match result {
            Ok(_) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.into(),
        }
    }
}
