/// Configures the global allocator based on the target environment.
///
/// On Windows (MSVC target), uses the `mimalloc` allocator for efficient memory management.
/// On other platforms, uses the `jemallocator` allocator for efficient memory management.
///
/// # Example
///
/// To use this global allocator, simply compile your Rust program with the correct target environment.
/// For example, on Windows:
///
/// ```bash
/// rustc --target x86_64-pc-windows-msvc program.rs
/// ```
///
/// And on other platforms:
///
/// ```bash
/// rustc --target x86_64-unknown-linux-gnu program.rs
/// ```
#[cfg(target_env = "msvc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;
