# RustZone TA

## Build

1. Install Rust for your platform and target using [rustup](https://rustup.rs/).
2. Install OP-TEE for your target platform using [OP-TEE build repository](https://github.com/OP-TEE/build)
3. Modify generate-ta.sh to point to your OP-TEE install 
4. Finally: `cd ta && ./generate-ta.sh`

This code is tested with the default QEMU target, and will likely require modifications to run on other targets.
