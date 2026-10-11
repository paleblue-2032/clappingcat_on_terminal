{
  description = "clpcat — ターミナルで拍手する黒猫。タイポを煽る";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;

      perSystem =
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          clpcat = pkgs.rustPlatform.buildRustPackage {
            pname = "clpcat";
            version = "0.1.0";
            src = pkgs.lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;

            meta = {
              description = "A clapping cat that dances in your terminal to mock your typos";
              homepage = "https://github.com/paleblue-2032/clappingcat_on_terminal";
              mainProgram = "clpcat";
              license = pkgs.lib.licenses.mit;
              platforms = pkgs.lib.platforms.unix;
            };
          };
        in
        {
          packages = {
            inherit clpcat;
            default = clpcat;
          };
          apps.default = {
            type = "app";
            program = "${clpcat}/bin/clpcat";
          };
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              rustc
            ];
          };
          formatter = pkgs.nixpkgs-fmt;
        };
    in
    {
      packages = forAllSystems (system: (perSystem system).packages);
      apps = forAllSystems (system: (perSystem system).apps);
      devShells = forAllSystems (system: (perSystem system).devShells);
      formatter = forAllSystems (system: (perSystem system).formatter);
    };
}
