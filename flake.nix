{
  description = "The flake that is used add Rust and it's packages the dev shell.";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs: with inputs; flake-utils.lib.eachDefaultSystem (system:
    let

      overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
      pkgs = import nixpkgs { inherit system; overlays = [ cargo2nix.overlays.default ]; };
      rustPkgs = pkgs.rustBuilder.makePackageSet { 
        rustVersion = "1.75.0"; 
        packageFun = import ./Cargo.nix; 
      };

    in rec {

      # ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Nix Build ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ #

      packages = {
        default = packages.tic_tac_rust;
        tic_tac_rust = (rustPkgs.workspace.tic_tac_rust {});
      };

      # ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Nix Flake Check ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ #

      checks = {

        cargo = pkgs.stdenv.mkDerivation {
          name = "cargo";
          src = ./.; 
          nativeBuildInputs = with pkgs; [ cargo ];
          buildPhase = "cargo check";
          installPhase = "mkdir $out";
        };

      };

      # ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Nix Develop ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ #

      devShells.default = pkgs.mkShell {
        
        RUSTC_VERSION = overrides.toolchain.channel;
        RUST_BACKTRACE = "full";

        buildInputs = with pkgs; [ 
          cargo 
          rustc 
          rustup 
          rust-analyzer 
          pkg-config 
          openssl 
          cargo-watch 
          clippy
        ];
        
        shellHook = let 

          shell-hook-text = (can-display-colors: let 
          
            black  = ( ''\033[1;30m'' );
            red    = ( ''\033[1;31m'' );
            green  = ( ''\033[1;32m'' );
            yellow = ( ''\033[1;33m'' );
            blue   = ( ''\033[1;34m'' );
            purple = ( ''\033[1;35m'' );
            cyan   = ( ''\033[1;36m'' );
            white  = ( ''\033[0;37m'' );

            color  = (string: color: if (can-display-colors == true) 
              then "${color}${string}${white}" 
              else string
            );

          in ''
            echo -e "\nDetected ${color system cyan}: started a ${color "Rust" red} development Shell powered by ${color "Nix" blue}."
            echo -e "Using ${color "$(cargo --version)" yellow} together with ${color "$(rustc --version)" yellow}."
            export PS1="(${color "Dev-Shell" purple}) $PS1"

            echo -e "\nPerforming ${color "nix flake check" blue} to ensure correctness:"
            nix flake check --all-systems

            echo -e "\nAll checks done!"
          '');

        in (''
          # if the terminal supports color. display the message with color, else just use black and white.
          if [[ -n "$(tput colors)" && "$(tput colors)" -gt 2 ]]; then
            ${shell-hook-text true}
          else 
            ${shell-hook-text false}
          fi''
        );
      };

      # ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ #
      
    }
  );
}