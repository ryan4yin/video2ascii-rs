{
  description = "Build a cargo project with a custom toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      # NB: we don't need to overlay our custom toolchain for the *entire*
      # pkgs (which would require rebuilding anything else which uses rust).
      # Instead, we just want to update the scope that crane will use by appending
      # our specific toolchain there.
      craneLib =
        (crane.mkLib pkgs).overrideToolchain (p:
          p.rust-bin.stable.latest.default);

      nativeBuildInputs = with pkgs; [
        llvmPackages_16.libcxxClang
        opencv

        vcpkg
        cmake
        pkg-config
      ];
      LIBCLANG_PATH = "${pkgs.llvmPackages_16.clang-unwrapped.lib}/lib";

      video2ascii-rs = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./.;
        strictDeps = true;

        inherit nativeBuildInputs LIBCLANG_PATH;
        buildInputs = with pkgs;
          [
            # Add additional build inputs here
            mpv
            opencv
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
          ];
      };
    in {
      checks = {
        inherit video2ascii-rs;
      };

      packages.default = video2ascii-rs;

      devShells.default = craneLib.devShell {
        # Inherit inputs from checks.
        checks = self.checks.${system};

        shellHook = ''
          export LIBCLANG_PATH=${LIBCLANG_PATH}
          cargo version
        '';

        # Extra inputs can be added here; cargo and rustc are provided by default
        # from the toolchain that was specified earlier.
        packages = with pkgs; [
          rust-analyzer
        ] ++ nativeBuildInputs;
      };
    });
}
