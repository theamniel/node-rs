/// Configures the global allocator based on the target environment.
#[cfg(target_os = "windows")]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[cfg(all(not(target_os = "windows"), not(target_env = "msvc")))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
