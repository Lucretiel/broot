 
# WARNING: This script is NOT meant for normal installation, it's dedicated
# to the compilation of all supported targets. This is a long process and
# it involves specialized toolchains.
# For usual compilation do
#     cargo build --release
# or read all possible installation solutions on
# https://dystroy.org/broot/documentation/installation/

# clean previous build
rm -rf build
mkdir build

# build the linux version
cargo build --release
strip target/release/broot
mkdir build/x86_64-linux/
cp target/release/broot build/x86_64-linux/

# find and copy the completion scripts
# (they're built as part of the normal compilation)
mkdir build/completion
cp "$(broot -c ":gi release :focus broot.bash :parent :pp" target)/"* build/completion

# build the windows version
# You need first to install the proper cargo toolchain:
# rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
mkdir build/x86_64-pc-windows-gnu/
cp target/x86_64-pc-windows-gnu/release/broot.exe build/x86_64-pc-windows-gnu/

# build the Raspberry version
# In order for this to work, you need to install the proper cargo toolchain
# and a linker:
#  rustup target add armv7-unknown-linux-gnueabihf 
#  sudo apt-get install gcc-8-multilib-arm-linux-gnueabihf
RUSTFLAGS="-C linker=arm-linux-gnueabihf-gcc-8" cargo build --target armv7-unknown-linux-gnueabihf --release
mkdir build/armv7-unknown-linux-gnueabihf
cp target/armv7-unknown-linux-gnueabihf/release/broot build/armv7-unknown-linux-gnueabihf/

