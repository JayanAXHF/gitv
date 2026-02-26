{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    packages = [
      nil
      alejandra

      rustc
      cargo
      clippy
      rustfmt
      rust-analyzer

      typos
    ];

    env = {
      RUST_BACKTRACE = "full";
    };

    shellHook = ''
      echo "Setting up pre-push hook..."

      HOOK=".git/hooks/pre-push"

      if [ ! -f "$HOOK" ]; then
        cp ./etc/pre-push.sh "$HOOK"
        chmod 755 "$HOOK"
        echo "Pre-push hook installed."
      fi
    '';
  }
