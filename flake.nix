{
  # paradigm's dev shell: pklith's shell (hk, pkl, git, pklith and every
  # tool the generated nix/pklith.nix names), plus the Rust toolchain for
  # the `dev` and `sherdwright` crates. One nixpkgs, followed from
  # nixpkgs-lock rather than spelled here (root V20).
  description = "paradigm -- the fleet lifecycle, written once and checked";

  # hk is built by nix-hk and pushed to this cache; without it the first
  # shell builds hk from source.
  nixConfig = {
    extra-substituters = [ "https://pr0d1r2.cachix.org" ];
    extra-trusted-public-keys = [
      "pr0d1r2.cachix.org-1:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM="
    ];
  };

  inputs = {
    nixpkgs-lock.url = "git+https://github.com/pr0d1r2/nixpkgs-lock?ref=main&shallow=1";
    nixpkgs.follows = "nixpkgs-lock/nixpkgs";
    pklith.url = "git+https://github.com/pr0d1r2/pklith?ref=main&shallow=1";
    pklith.inputs.nixpkgs-lock.follows = "nixpkgs-lock";
    # The CI materializer (root §C, V28): renders .github/workflows/ from
    # .actlith, and its gen --check keeps the two from drifting.
    actlith.url = "git+https://github.com/pr0d1r2/actlith?ref=main&shallow=1";
    actlith.inputs.nixpkgs-lock.follows = "nixpkgs-lock";
  };

  outputs =
    {
      nixpkgs,
      pklith,
      actlith,
      ...
    }:
    let
      # pklith's lib.devShell supports these three; it names them when asked
      # for another.
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAll = f: nixpkgs.lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
      # The spec tools from pklith's catalog, so the spec gate (root V9) has
      # them before `.pklith` asks for them.
      specTools =
        pkgs:
        let
          catalog = pklith.legacyPackages.${pkgs.stdenv.hostPlatform.system}.catalog;
        in
        [
          catalog.microlith
          catalog.sherd
          catalog.itok
        ];
    in
    {
      devShells = forAll (pkgs: {
        # cargo-llvm-cov finds llvm-cov and llvm-profdata through these, since
        # nixpkgs' rustc ships no llvm-tools-preview.
        default =
          (pklith.lib.devShell {
            inherit pkgs;
            src = ./.;
            packages = [
              pkgs.rustc
              pkgs.cargo
              pkgs.clippy
              pkgs.rustfmt
              pkgs.cargo-llvm-cov
              pkgs.llvmPackages.llvm
              pkgs.gh
              actlith.packages.${pkgs.stdenv.hostPlatform.system}.default
            ]
            ++ specTools pkgs;
          }).overrideAttrs
            {
              LLVM_COV = "${pkgs.llvmPackages.llvm}/bin/llvm-cov";
              LLVM_PROFDATA = "${pkgs.llvmPackages.llvm}/bin/llvm-profdata";
            };
      });
    };
}
