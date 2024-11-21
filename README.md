## Intro

A Prometheus exporter for Seagate drive S.M.A.R.T. metrics that parses [openSeaChest](https://github.com/Seagate/openSeaChest) tool output.

Built with the rust Prometheus exporter framework, [prometheus_exporter_base](https://github.com/MindFlavor/prometheus_exporter_base/tree/master).

### Grafana dashboard included
![Seagate drive S.M.A.R.T. warranty metrics](resources/openseachest_dashboard_1.png?raw=true "Seagate drive S.M.A.R.T. warranty metrics")
![Seagate drive S.M.A.R.T. physical property metrics](resources/openseachest_dashboard_1.png?raw=true "Seagate drive S.M.A.R.T. physical property metrics")

## Setup

### Build from source

1. You need [Rust](https://www.rust-lang.org/tools/install) installed.
1. You need [openSeaChest](https://github.com/Seagate/openSeaChest) tools installed.
1. Clone the repository with

    ```sh
    git clone https://github.com/joshpearce/openseachest_exporter.git
    cd openseachest_exporter
    ```

1. Compile the program with

    ```sh
    cargo install --path .
    ```

1. Location where cargo installed the program and run it with the `-h` flag to see all options.

    ```sh
    $ /home/josh/.cargo/bin/openseachest_exporter -h
    Usage: openseachest_exporter [OPTIONS] --opensea-smart-bin <OPENSEA_SMART_BIN>

    Options:
        --opensea-smart-bin <OPENSEA_SMART_BIN>
            Path to openSeaChest_SMART binary [env: OPENSEA_SMART_BIN=]
        --listen <LISTEN>
            IPv4/6 socket+port address for HTTP listener [env: OPENSEA_SMART_LISTEN=] [default: 0.0.0.0:10988]
        --log-level <LOG_LEVEL>
            Path to openSeaChest_SMART binary [env: OPENSEA_SMART_LOG_LEVEL=] [default: error] [possible values: trace, debug, info, warn, error]
        --host-name <HOST_NAME>
            Hostname for metrics [env: HOST=] [default: no_host_name]
        --map-file <MAP_FILE>
            JSON file for renaming metrics [env: OPENSEA_SMART_MAP_FILE=]
    -h, --help
            Print help
    ```

1. Check it's up by visiting [http://127.0.0.1:10988/metrics](http://127.0.0.1:10988/metrics)
