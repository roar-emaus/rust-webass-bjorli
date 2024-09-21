{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # List of dependencies to be included in the shell environment
  buildInputs = [
    pkgs.rustup     # Rust toolchain manager
    pkgs.cargo      # Rust package manager
    pkgs.wasm-pack  # Tool to build WebAssembly package from Rust
    pkgs.nodejs     # Node.js for JavaScript integration and testing
  ];

  # Optional: Automatically set up the Rust toolchain
  shellHook = ''
    # Install the latest stable version of Rust if not already installed
    if ! rustup toolchain list | grep -q "stable"; then
      rustup install stable
    fi

    rustup default stable

    # Add the wasm32 target if not already added
    rustup target add wasm32-unknown-unknown

    echo "Rust WebAssembly environment ready!"
  '';
}
