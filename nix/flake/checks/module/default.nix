{
  lib,
  nixos,
  runCommand,
  self,
}:
let
  system = nixos {
    imports = [ self.nixosModules.default ];
    baton = {
      enable = true;
      locks.rtx3080 = {
        device = "0000:03:00.0";
        applications = [ "guest" ];
        defaultOwner = "desktop";
        yieldingDefaultOwner = true;
      };
    };
    boot.loader.grub.enable = false;
    fileSystems."/" = {
      device = "none";
      fsType = "tmpfs";
    };
    system.stateVersion = "26.05";
  };
  inherit (system) config;
  sudoRules = lib.concatMapStringsSep "\n" (
    rule: lib.concatMapStringsSep "\n" (command: command.command) rule.commands
  ) (lib.filter (rule: rule ? groups && rule.groups == [ "baton" ]) config.security.sudo.extraRules);
in
runCommand "module"
  {
    udevRules = config.services.udev.extraRules;
    inherit sudoRules;
    activationText = config.system.activationScripts.baton.text;
    batonProgram = config.baton.program;
    hostProgram = config.baton.hostProgram;
    passAsFile = [
      "udevRules"
      "sudoRules"
      "activationText"
    ];
  }
  ''
    fail() {
      echo "[MODULE] $1"
      exit 1
    }
    grep --quiet 'KERNEL=="0000:03:00.0"' "$udevRulesPath" || fail "the rule of the udev of the lock rtx3080 is missing."
    grep --quiet 'KERNELS=="0000:03:00.\*"' "$udevRulesPath" || fail "the rule of the udev that refreshes the slot 0000:03:00 is missing."
    enclosure=$(grep --only-matching 'RUN+="[^"]*baton-enclosure-rtx3080"' "$udevRulesPath" | cut --delimiter '"' --fields 2)
    grep --quiet -- "enclose --lock rtx3080 --device 0000:03:00.0" "$enclosure" || fail "the enclosure does not enclose the GPU."
    grep --quiet -- "--application guest 2" "$enclosure" || fail "the enclosure does not admit guest."
    grep --quiet -- "--application desktop --default-owner --yielding" "$enclosure" || fail "the enclosure does not admit desktop as the default owner."
    test "$(grep --count "" "$sudoRulesPath")" = 3 || fail "the sudo rules allow more than acquire, release and status."
    grep --quiet 'baton_host status$' "$sudoRulesPath" || fail "the sudo rules do not allow the status."
    grep --quiet 'baton-admission-rtx3080' "$activationTextPath" || fail "the activation does not admit the applications."
    grep --quiet 'BATON_HOST_PROGRAM' "$batonProgram" || fail "baton is not wrapped with the path of baton_host."
    host=$(grep --only-matching 'RUN+="[^"]*baton_host refresh"' "$udevRulesPath" | cut --delimiter '"' --fields 2 | cut --delimiter ' ' --fields 1)
    grep --quiet -- "$hostProgram acquire" "$sudoRulesPath" || fail "baton.hostProgram is not the program sudo allows."
    grep --quiet 'fbset' "$host" || fail "baton_host is not wrapped with the path of con2fbmap."
    echo "[MODULE] the module encloses every lock at the bind of its driver, refreshes its slot and allows only acquire, release and status."
    touch "$out"
  ''
