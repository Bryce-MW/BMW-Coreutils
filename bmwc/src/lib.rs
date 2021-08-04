// NOTE(bryce): Eventually make our own allocator!
//  Jemalloc appears to have some issues and it also not Rust
use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
