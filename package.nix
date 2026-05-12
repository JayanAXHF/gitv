{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "gitv";
  version = "dev";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  env = {
    VERGEN_GIT_DESCRIBE = finalAttrs.version;
    VERGEN_BUILD_DATE = "unknown";
  };

  meta = {
    description = "Terminal-based viewer for GitHub issues";
    homepage = "https://github.com/JayanAXHF/gitv";
    license = with lib.licenses; [mit unlicense];
  };
})
