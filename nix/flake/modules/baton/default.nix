{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib)
    attrNames
    concatMapStringsSep
    concatStringsSep
    escapeShellArg
    mapAttrsToList
    mkEnableOption
    mkIf
    mkOption
    optional
    types
    unique
    ;
  setting = config.baton;
  flakePackages = self.packages.${pkgs.stdenv.hostPlatform.system};
  hostProgram = "${setting.hostPackage}/bin/baton_host";
  elevationProgram = "/run/wrappers/bin/sudo";
  program =
    pkgs.runCommand "baton"
      {
        nativeBuildInputs = [ pkgs.makeWrapper ];
      }
      ''
        makeWrapper ${setting.package}/bin/baton $out/bin/baton \
          --set-default BATON_ELEVATION_PROGRAM ${elevationProgram} \
          --set-default BATON_HOST_PROGRAM ${hostProgram}
      '';
  lockApplications =
    lock: unique (lock.applications ++ optional (lock.defaultOwner != null) lock.defaultOwner);
  admissionLines =
    lockName: lock:
    concatMapStringsSep "\n" (
      application:
      let
        defaultFlag = if application == lock.defaultOwner then " --default-owner" else "";
      in
      "${hostProgram} admit --lock ${escapeShellArg lockName} --application ${escapeShellArg application}${defaultFlag} 2>/dev/null || true"
    ) (lockApplications lock);
  enclosureProgram =
    lockName: lock:
    pkgs.writeShellScript "baton-enclosure-${lockName}" ''
      if ${hostProgram} users --lock ${escapeShellArg lockName} >/dev/null 2>&1; then
        exec ${hostProgram} refresh
      fi
      ${hostProgram} enclose --lock ${escapeShellArg lockName} --device ${escapeShellArg lock.device} >/dev/null
      ${admissionLines lockName lock}
    '';
  admissionProgram =
    lockName: lock:
    pkgs.writeShellScript "baton-admission-${lockName}" ''
      if ${hostProgram} users --lock ${escapeShellArg lockName} >/dev/null 2>&1; then
        ${admissionLines lockName lock}
      fi
    '';
  slotText = device: lib.head (lib.splitString "." device);
  udevLines =
    lockName: lock:
    concatStringsSep "\n" [
      ''ACTION=="add|bind", SUBSYSTEM=="pci", KERNEL=="${lock.device}", DRIVER=="?*", DRIVER!="vfio-pci", RUN+="${enclosureProgram lockName lock}"''
      ''ACTION=="add", KERNELS=="${slotText lock.device}.*", ENV{DEVNAME}=="?*", RUN+="${hostProgram} refresh"''
    ];
  lockType = types.submodule {
    options = {
      device = mkOption {
        type = types.strMatching "[0-9a-f]{4}:[0-9a-f]{2}:[0-9a-f]{2}\\.[0-7]";
        example = "0000:03:00.0";
        description = "The PCI address of the GPU the lock holds, with every function of its slot.";
      };
      applications = mkOption {
        type = types.listOf types.str;
        default = [ ];
        example = [
          "render"
          "guest"
        ];
        description = "The eligible applications of the lock, every application that may use its GPU at all.";
      };
      defaultOwner = mkOption {
        type = types.nullOr types.str;
        default = null;
        example = "desktop";
        description = "The eligible application that holds the lock whenever no other does, the desktop on a system with a single GPU.";
      };
    };
  };
in
{
  options.baton = {
    enable = mkEnableOption "baton, a mutex around GPUs";
    package = mkOption {
      type = types.package;
      default = flakePackages.baton;
      description = "The package of the program baton.";
    };
    hostPackage = mkOption {
      type = types.package;
      default = flakePackages.baton_host;
      description = "The package of the helper baton_host, which runs as root.";
    };
    group = mkOption {
      type = types.str;
      default = "baton";
      description = "The group whose members acquire, release and read the locks through baton.";
    };
    locks = mkOption {
      type = types.attrsOf lockType;
      default = { };
      description = "The locks of the host, each named for its GPU, such as rtx3080.";
    };
    program = mkOption {
      type = types.str;
      readOnly = true;
      default = "${program}/bin/baton";
      description = "The path of baton, wrapped to reach baton_host through sudo, for every program that runs inside a lock.";
    };
  };

  config = mkIf setting.enable {
    assertions =
      map (lockName: {
        assertion = lockName != "host";
        message = "baton.locks.host is no lock, host names the cgroup of the helper.";
      }) (attrNames setting.locks)
      ++ mapAttrsToList (lockName: lock: {
        assertion = lockApplications lock != [ ];
        message = "baton.locks.${lockName} names no application, so nothing could ever use its GPU.";
      }) setting.locks;
    environment.systemPackages = [ program ];
    users.groups.${setting.group} = { };
    security.sudo.extraRules = [
      {
        groups = [ setting.group ];
        commands =
          map
            (command: {
              inherit command;
              options = [ "NOPASSWD" ];
            })
            [
              "${hostProgram} acquire *"
              "${hostProgram} release *"
              "${hostProgram} status"
            ];
      }
    ];
    services.udev.extraRules = concatStringsSep "\n" (mapAttrsToList udevLines setting.locks);
    system.activationScripts.baton = {
      text = concatStringsSep "\n" (
        mapAttrsToList (lockName: lock: "${admissionProgram lockName lock}") setting.locks
      );
      deps = [ ];
    };
  };
}
