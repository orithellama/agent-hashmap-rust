use std::fmt;
use std::io;
use std::path::PathBuf;

use thiserror::Error;

/// Crate-local result alias.
///
/// Keeping a single result alias across the crate makes signatures easier to
/// read and keeps error handling consistent.
pub type Result<T> = std::result::Result<T, AgentMemoryError>;

#[derive(Debug, Error)]
pub enum AgentMemoryError {
    /// A caller supplied invalid input.
    #[error(transparent)]
    Validation(#[from] ValidationError),

    /// The on-disk configuration was invalid or incomplete.
    #[error(transparent)]
    Config(#[from] ConfigError),

    /// A storage operation failed.
    #[error(transparent)]
    Store(#[from] StoreError),

    /// A requested item does not exist.
    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    /// An unexpected I/O error occurred outside a more specific storage context.
    #[error("I/O error: {source}")]
    Io {
        #[source]
        source: io::Error,
    },

    /// A size or capacity calculation would overflow or exceed a defined limit.
    #[error("overflow while {context}")]
    Overflow {
        /// Human-readable description of the failing operation.
        context: &'static str,
    },

    /// An internal invariant was violated.
    ///
    /// It is intended for situations where the crate detects 
    //  a state that "should never happen" if the implementation is correct.
    #[error("internal invariant violation: {message}")]
    Internal {
        /// Human-readable invariant description.
        message: &'static str,
    },
}

impl AgentMemoryError {
    /// Convenience constructor for generic I/O failures.
    #[must_use]
    pub fn io(source: io::Error) -> Self {
        Self::Io { source }
    }

    /// Convenience constructor for overflow-related failures.
    #[must_use]
    pub const fn overflow(context: &'static str) -> Self {
        Self::Overflow { context }
    }

    /// Convenience constructor for internal invariant violations.
    #[must_use]
    pub const fn internal(message: &'static str) -> Self {
        Self::Internal { message }
    }
}

impl From<io::Error> for AgentMemoryError {
    fn from(source: io::Error) -> Self {
        Self::Io { source }
    }
}

/// Validation failures for user input, keys, namespaces, values and paths.
///
/// These errors should be raised as early as possible, before any mutation or
/// persistence side effect occurs.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// A required field was empty.
    #[error("{field} must not be empty")]
    Empty {
        /// Logical field name, such as `key` or `project_name`.
        field: &'static str,
    },

    /// The provided value exceeded an allowed length.
    #[error("{field} exceeds maximum length: actual={actual}, max={max}")]
    TooLong {
        /// Logical field name.
        field: &'static str,
        /// Actual observed length.
        actual: usize,
        /// Maximum allowed length.
        max: usize,
    },

    /// The provided value was shorter than a minimum required length.
    #[error("{field} is below minimum length: actual={actual}, min={min}")]
    TooShort {
        /// Logical field name.
        field: &'static str,
        /// Actual observed length.
        actual: usize,
        /// Minimum required length.
        min: usize,
    },

    /// The value contained an invalid character for the target field.
    #[error("{field} contains invalid character {character:?} at byte index {index}")]
    InvalidCharacter {
        /// Logical field name.
        field: &'static str,
        /// The offending character.
        character: char,
        /// Byte index into the original string.
        index: usize,
    },

    /// The value contained an invalid byte sequence or unsupported encoding.
    #[error("{field} contains invalid encoding")]
    InvalidEncoding {
        /// Logical field name.
        field: &'static str,
    },

    /// A path failed validation.
    #[error("invalid path for {field}: {reason}")]
    InvalidPath {
        /// Logical field name.
        field: &'static str,
        /// Human-readable validation reason.
        reason: &'static str,
    },

    /// A namespace or key segment was malformed.
    #[error("invalid segment in {field}: {reason}")]
    InvalidSegment {
        /// Logical field name.
        field: &'static str,
        /// Human-readable validation reason.
        reason: &'static str,
    },

    /// The input was structurally invalid.
    #[error("invalid {field}: {reason}")]
    InvalidFormat {
        /// Logical field name.
        field: &'static str,
        /// Human-readable validation reason.
        reason: &'static str,
    },
}

impl ValidationError {
    /// Constructs an `Empty` validation error.
    #[must_use]
    pub const fn empty(field: &'static str) -> Self {
        Self::Empty { field }
    }

    /// Constructs a `TooLong` validation error.
    #[must_use]
    pub const fn too_long(field: &'static str, actual: usize, max: usize) -> Self {
        Self::TooLong { field, actual, max }
    }

    /// Constructs a `TooShort` validation error.
    #[must_use]
    pub const fn too_short(field: &'static str, actual: usize, min: usize) -> Self {
        Self::TooShort { field, actual, min }
    }

    /// Constructs an `InvalidPath` validation error.
    #[must_use]
    pub const fn invalid_path(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidPath { field, reason }
    }

    /// Constructs an `InvalidSegment` validation error.
    #[must_use]
    pub const fn invalid_segment(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidSegment { field, reason }
    }

    /// Constructs an `InvalidFormat` validation error.
    #[must_use]
    pub const fn invalid_format(field: &'static str, reason: &'static str) -> Self {
        Self::InvalidFormat { field, reason }
    }
}

/// Configuration-specific failures.
///
/// These errors are intentionally separate from generic validation failures so
/// callers can distinguish:
///
/// - "the user typed a bad key"
/// - "the store config on disk is invalid"
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// A required configuration field was missing.
    #[error("missing required configuration field: {field}")]
    MissingField {
        /// Field name as represented in config.
        field: &'static str,
    },

    /// A configuration version is unsupported.
    #[error("unsupported configuration version: {version}")]
    UnsupportedVersion {
        /// Parsed configuration version.
        version: u32,
    },

    /// The configuration file was malformed.
    #[error("malformed configuration: {reason}")]
    Malformed {
        /// Human-readable failure reason.
        reason: &'static str,
    },

    /// The configured project name failed validation.
    #[error("invalid project name: {reason}")]
    InvalidProjectName {
        /// Human-readable failure reason.
        reason: &'static str,
    },

    /// The configured store path failed validation.
    #[error("invalid store path: {reason}")]
    InvalidStorePath {
        /// Human-readable failure reason.
        reason: &'static str,
    },

    /// The configuration file could not be parsed.
    #[error("failed to parse configuration: {message}")]
    Parse {
        /// Parser-facing or user-facing message.
        message: String,
    },
}

/// Errors produced by the store and persistence layers.
///
/// These are the failures most likely to surface during normal operation:
/// opening a store, loading it, flushing it, or acquiring a lock.
#[derive(Debug, Error)]
pub enum StoreError {
    /// The store path has not been initialized yet.
    #[error("store is not initialized")]
    NotInitialized,

    /// The storage file does not exist.
    #[error("store file does not exist: {path}")]
    MissingFile {
        /// Missing storage file path.
        path: PathBuf,
    },

    /// The store file was malformed.
    #[error("store file is malformed: {reason}")]
    Malformed {
        /// Human-readable reason.
        reason: String,
    },

    /// The store file uses an unsupported version.
    #[error("unsupported store format version: {version}")]
    UnsupportedVersion {
        /// Parsed version.
        version: u32,
    },

    /// The storage path could not be created or prepared.
    #[error("failed to prepare store path {path}: {source}")]
    PreparePath {
        /// Path that failed preparation.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// Failed to read from storage.
    #[error("failed to read store {path}: {source}")]
    Read {
        /// Store path.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// Failed to write to storage.
    #[error("failed to write store {path}: {source}")]
    Write {
        /// Store path.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// Failed while atomically persisting data to a temporary file or during
    /// the final rename.
    #[error("atomic persist failed for {path}: {reason}")]
    AtomicPersist {
        /// Target path.
        path: PathBuf,
        /// Human-readable reason.
        reason: String,
    },

    /// A lock could not be acquired or maintained.
    #[error(transparent)]
    Lock(#[from] LockError),

    /// Serialization failed.
    #[error("failed to serialize store: {message}")]
    Serialize {
        /// Human-readable error message.
        message: String,
    },

    /// Deserialization failed.
    #[error("failed to deserialize store: {message}")]
    Deserialize {
        /// Human-readable error message.
        message: String,
    },
}

impl StoreError {
    /// Constructs a `MissingFile` error.
    #[must_use]
    pub fn missing_file(path: impl Into<PathBuf>) -> Self {
        Self::MissingFile { path: path.into() }
    }

    /// Constructs a `Malformed` store error.
    #[must_use]
    pub fn malformed(reason: impl Into<String>) -> Self {
        Self::Malformed {
            reason: reason.into(),
        }
    }

    /// Constructs a `Read` store error.
    #[must_use]
    pub fn read(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Read {
            path: path.into(),
            source,
        }
    }

    /// Constructs a `Write` store error.
    #[must_use]
    pub fn write(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Write {
            path: path.into(),
            source,
        }
    }

    /// Constructs an `AtomicPersist` store error.
    #[must_use]
    pub fn atomic_persist(path: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::AtomicPersist {
            path: path.into(),
            reason: reason.into(),
        }
    }
}

/// Locking failures.
///
/// A dedicated type keeps locking concerns explicit and helps later if the
/// crate supports multiple backends or operating-system-specific lock strategies.
#[derive(Debug, Error)]
pub enum LockError {
    /// The lock is already held by another process or owner.
    #[error("store lock is already held: {path}")]
    AlreadyHeld {
        /// Lock path or related store path.
        path: PathBuf,
    },

    /// The lock operation timed out.
    #[error("timed out while acquiring lock: {path}")]
    Timeout {
        /// Lock path or related store path.
        path: PathBuf,
    },

    /// The lock could not be acquired due to an I/O failure.
    #[error("failed to acquire lock for {path}: {source}")]
    Acquire {
        /// Lock path or related store path.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// The lock could not be released cleanly.
    #[error("failed to release lock for {path}: {source}")]
    Release {
        /// Lock path or related store path.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },
}

/// A domain-level not-found condition.
///
/// This is separate from `Option<T>`-style lookup misses because some API calls
/// may want to return a structured error when a user explicitly requested an
/// entity that should exist.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("{kind} not found: {identifier}")]
pub struct NotFoundError {
    /// Logical entity kind, such as `key`, `namespace`, or `store`.
    pub kind: &'static str,
    /// Human-readable identifier.
    pub identifier: String,
}

impl NotFoundError {
    /// Constructs a new not-found error.
    #[must_use]
    pub fn new(kind: &'static str, identifier: impl Into<String>) -> Self {
        Self {
            kind,
            identifier: identifier.into(),
        }
    }
}

impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as std::error::Error>::fmt(self, f)
    }
}