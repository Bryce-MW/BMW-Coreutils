# BMW Coreutils
There are a couple main goals of this repository. I want to provide an implementation of at least libc and libm written
entirely in rust (and likely some inline assembly on some platforms). This must be done carefully because rust itself
depends on those libraries. Our versions will probably have to be I am not yet entirely sure how to confirm if a Rust
binary or C library has been linked to the system libc or my own. I have also tentatively put in libresolv, libiconv,
and libSystem for testing purposes. While I do wish to remove as much dependence on C as possible, libresolv and
libiconv are not part of the C standard library so they are less important. libSystem is a part of macOS's system
library. I am not planning to replace system libraries such as libSystem for macOS or the various Windows DLLs. The
issue is that on macOS, libSystem also contains the rest of the C standard library and I don't want to accidentally
include that.

Eventually, I want to make it so that these libraries can be included as regular Rust libraries (with extra
functionality) as well as compiled to native C libraries. This is possible already since the symbols from our libraries
will overload those from the standard libraries. It's not possible to tell Rust not to include those system libraries
though so in order to insure they are not included, I use a custom build script to compile them to native libraries and
link them in with higher priority. You are free to include them yourself but you won't get any extra features for now.
If you want to use these libraries directly, I recommend using the libc crate. This means that you can add bmwc as an
optional dependency which will replace the C libraries that the libc crate links to when selected.

I do acknowledge that RIIR (Rewrite It In Rust) is a bit of a joke these days. While I think I do have some good ideas
for actual improvements, a simple port of these libraries and utilities to rust is not useful to the community though it
is useful for my own education. On the note of licensing, I am not a lawyer but I do believe that with the current
license, I would not be able to use any code from GNU Coreutils. I am not planning to but I wanted to note that while I
have done a cursory glance at the implementation of a couple of their utilities in the past, I don't remember much. I
will be keeping all of my code original and not looking at any of their code in the future. I will also be sticking to
using the official POSIX docs for determining the requirements for each utility. Extending the utilities to allow GNU
flags may be possible in the future.

Just one more note, if you wish to install one of these binaries, use `cargo install bmw_coreutils --bin [UTILITY]`.
Otherwise you will install all of them which you may not want. Once the library is done enough that it can compile, the
library will be used by default. To disable this, you can add the `--no-default-features` flag to cargo.

## Thoughts on implementation
* Right now I am using [clap](https://crates.io/crates/clap) for the command-line argument passing. Most of the
  utilities are very small and generally take very few flags so it may be better for us to parse them by ourselves. It
  would be nice to keep the utilities about as light as their C versions. I would also love to look at the performance
  since there are a number of interesting articles describing how to get better performance out of standard utilities.

* I want the utilities (and libraries) to be mostly cross platform, at least on macOS, Windows, and Linux at the start.
  Anyone on one of those platforms should be able to use cargo install to get theses utilities. Some utilities may not
  be available to those on some platforms or may work differently. For example, Windows uses ACLs for everything so
  `chmod` and `chown` will have different possibly somewhat confusing results.
  
* While I am willing to use Rust libraries, I do wish to avoid using Rust libraries that link into C libraries since the
  point of this repository is to avoid Rust.
  
* I am going to write my own memory allocator. This will be available both through the malloc interface so that it can
  be used from C, as well as the Rust global allocator interface since Rust can make use of features like sized frees
  that significantly improve performance of reallocation in the design that I am going for. My design is to keep
  separate pages for difference sizes of allocations and have all allocations larger than a page directly allocate
  memory. I will describe my idea in more detail when I get to working on that but it works in a similar way to
  jemalloc.
  
* I want to use platform optimizations where possible. For example, macOS can use all of the POSIX APIs as you would
  expect but there are also macOS specific APIs that can provide better performance or are at least more semantic. The
  way that I am going to do this is by having a platform module (platform crate?) which provides a small API for common
  tasks like getting the page size and allocating pages of memory. It may be more abstract than that. Then the module
  will use conditional compilation to choose which platform module to export these functions from. The platform module
  can then use whatever it wants that does this the best. In the example of allocating pages of memory, the platform
  could use mmap, vm_allocate, or VirtualAlloc depending on the platform.
  
* If there are existing Rust-based utilities such as [bat](https://crates.io/crates/bat) (an alternative to cat), I may
  opt to recommend linking those instead. At the time of writing, other than bat, I also know of
  [ripgrep](https://crates.io/crates/ripgrep) though I don't know if it is compliant with the standard version of grep.
  There are probably many more that I will research later. It might be that I write my own version and later have to
  switch to one that I find. If you know of any exiting Rust alternates of the utility that I have implemented please
  open an issue.
  
* I would like to create an interface that shells and other programs can use to gain more information about a utility.
  This might be for seeing what is needed to make the utility purely POSIX compliant, for signaling extra features that
  shells can't rely on all executables having. As far as I am aware, most executables can actually export functions as
  if they were shared libraries. Perhaps that would work or we could use an alternate section of the executable.
  
* While it is intended to eventually be possible to use as a replacement for the GNU Coreutils in their entirety, for
  now, these will be optional versions that I recommend only using in interactive shells though I will try to ensure
  that when running non-interactively, they are mostly POSIX compliant but use them at your own risk.
  
* Building a shell is probably one of the last things that I will work on because it is a big program to write. That
  being said, I do have some ideas for possible improvements. One major improvement that I would like to make is to
  allow programs which specify this ability to be reentered. That is if a shell script (interactive shells will not
  do this) runs a program multiple times, it will only be loaded once and simply be given the arguments. I would also
  want to allow the shell to optionally have some more JQ-like interactions. The most basic would be piping output to
  multiple other programs.
  
* When building on Windows, I would like to eventually support fast pipes just in case anyone ever makes a full terminal
  out of Casey Muratori's [RefTerm](https://github.com/cmuratori/refterm). Perhaps we will be the ones who build this
  full terminal (porting to Rust of course).
  
## Implemented utilities
* echo

## Implemented library functions
