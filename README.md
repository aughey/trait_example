# trait_example

To dump out the intermediate representations like LLVM IR, MIR, HIR, and assembly code while building a Rust project, you can use various compiler flags with `rustc`, the Rust compiler. However, note that using these flags can be quite complex and is typically used for in-depth analysis or optimization.

Here's how you can dump each of these representations:

1. **MIR (Mid-level Intermediate Representation):**
   - Use the `-Z dump-mir` flag. This is only available on nightly Rust, so you'll need to use a nightly toolchain.
   - Example: `RUSTFLAGS="-Z dump-mir=all" cargo +nightly build`

2. **HIR (High-level Intermediate Representation):**
   - Rust doesn't provide a direct way to dump HIR. However, you can use tools like `rustc`'s internal debugging options with `-Z unpretty=hir` or `cargo rustc -- -Z unpretty=hir` on a nightly compiler.

3. **LLVM IR:**
   - Use the `--emit=llvm-ir` option with `rustc`.
   - Example: `cargo rustc -- --emit=llvm-ir`
   - This generates `.ll` files containing LLVM IR.

4. **Assembly Code:**
   - Use the `--emit=asm` option with `rustc`.
   - Example: `cargo rustc -- --emit=asm`
   - This generates `.s` files containing assembly code.

Remember, many of these flags and options are only available on the nightly Rust compiler and are primarily intended for compiler developers and advanced debugging. To use them, you'll need to install the nightly version of Rust using rustup (`rustup default nightly`) and be aware that these flags can change or be removed in future versions.

To view the dumped files, you can find them in the `target/debug/deps` directory (for a debug build) or `target/release/deps` (for a release build), along with the compiled output of your project. The exact location and naming of the files can vary based on your project structure and build configuration.

```
@aughey ➜ /workspaces/trait_example (main) $ RUSTFLAGS="-Z dump-mir=all" cargo +nightly build
error: toolchain 'nightly-x86_64-unknown-linux-gnu' is not installed
@aughey ➜ /workspaces/trait_example (main) $ rustup install nightly
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2024-01-13, rust version 1.77.0-nightly (2319be8e2 2024-01-12)
info: downloading component 'cargo'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: installing component 'cargo'
info: installing component 'rust-std'
 25.8 MiB /  25.8 MiB (100 %)  14.2 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 61.5 MiB /  61.5 MiB (100 %)  14.8 MiB/s in  4s ETA:  0s

  nightly-x86_64-unknown-linux-gnu installed - rustc 1.77.0-nightly (2319be8e2 2024-01-12)

info: checking for self-update
@aughey ➜ /workspaces/trait_example (main) $ 
```