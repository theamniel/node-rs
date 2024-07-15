/// Configures the global allocator based on the target environment.
#[cfg(all(not(target_os = "linux"), not(target_family = "wasm")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(target_os = "linux", not(target_family = "wasm")))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;