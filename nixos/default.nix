{flake}:
{ config, pkgs, lib, ... }:
let
  cfg = config.services.openseachest_exporter;

  serviceArgs = srv_cfg: ([
    "--opensea-smart-bin ${srv_cfg.package}/bin/openseachest_exporter"
    "--listen ${srv_cfg.listenAddress}"
    "--log-level ${srv_cfg.logLevel}"
    "--host-name ${srv_cfg.hostName}"
  ]);

in
{
  options.services.openseachest_exporter = with lib; {
    
    enable = mkEnableOption "openseachest_exporter service";

    package = mkOption {
      description = "Package to run openseachest_exporter out of";
      default = flake.packages.${pkgs.stdenv.targetPlatform.system}.openseachest_exporter;
      type = types.package;
    };

    openSeaSmartBinary = mkOption {
      description = "Path to openSeaChest_SMART binary.";
      type = types.path;
      default = "${pkgs.openseachest.outPath}/bin/openSeaChest_SMART.";
    };

    listenAddress = mkOption {
      description = "IPv4/6 socket+port address for HTTP listener.";
      type = types.str;
      example = "127.0.0.1:8080 or [::1]:8080";
      default = "0.0.0.0:10988";
    };

    logLevel = mkOption {
      description = "Log level for stderr output. One of: trace, debug, info, warn, error";
      type = types.strMatching "(trace|debug|info|warn|error)";
      default = "error";
    };

    hostName = mkOption {
      description = "Hostname to add to metric.";
      type = types.str;
      example = "my-machine";
    };

  };

  config = mkIf cfg.enable {
    systemd.services.openseachest_exporter = {
      wantedBy = [ "multi-user.target" ];
      after = ["network-online.target"];
      serviceConfig = {
        ExecStart = ''
          ${cfg.package}/bin/openseachest_exporter ${lib.escapeShellArgs (serviceArgs cfg)}
        '';
      };
    };
    
  };
}