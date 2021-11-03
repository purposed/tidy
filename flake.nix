{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, flake-compat }:
    utils.lib.eachDefaultSystem
      (system:
        let
          name = "tidy";
          lib = pkgs.lib;
          overlays = [
            rust-overlay.overlay
            (self: super: {
              rustc = self.rust-bin.stable.latest.default;
              cargo = self.rust-bin.stable.latest.default;
            })
          ];
          cargoNix = pkgs.callPackage ./Cargo.nix {
            inherit pkgs;
            release = true;
          };
          debugCargoNix = pkgs.callPackage ./Cargo.nix {
            inherit pkgs;
            release = false;
          };
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        rec {
          # `nix build`
          packages = lib.attrsets.mapAttrs
            (name: value: value.build)
            cargoNix.workspaceMembers;

          defaultPackage = self.packages.${system}.${name};

          # `nix run`
          apps =
            {
              tidy = utils.lib.mkApp
                {
                  name = "$name";
                  drv = self.packages.${system}.${name};
                };
            };

          checks = lib.attrsets.mapAttrs
            (name: value: value.build.override {
              runTests = true;
            })
            debugCargoNix.workspaceMembers //
          {
            format = pkgs.runCommand "format"
              {
                src = ./.;
                buildInputs = [ pkgs.rust-bin.stable.latest.default ];
              } ''
              mkdir $out
              cd $src
              cargo fmt -- --check
            '';
          };

          # `nix develop`
          devShell = pkgs.mkShell {
            buildInputs = [
              (pkgs.rust-bin.stable.latest.default.override
                {
                  extensions = [ "rust-src" "rust-analysis" ];
                })
            ];
          };
        });
}
