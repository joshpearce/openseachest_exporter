use clap::Parser;
use hyper::{Body, Request};
use log::{error, info};
use openseachest_exporter::{
    smart_data::{SmartAttribute, SmartDeviceInfo},
    smart_parsers::{parse_smart_attributes, parse_smart_scan},
};
use prometheus_exporter_base::{render_prometheus, MetricType, PrometheusInstance, PrometheusMetric, MissingValue};
use prometheus_exporter_base::prelude::{Authorization, ServerOptions};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::process::{Command, Stdio};
use std::env;

// Grab a port here one day: https://github.com/prometheus/prometheus/wiki/Default-port-allocations

#[derive(Parser, Debug, Clone)]
#[command(name = "openseachest_exporter")]
struct CliOptions {
    #[arg(
        long = "opensea-smart-bin",
        env = "OPENSEA_SMART_BIN",
        //default_value = "openSeaChest_SMART",
        help = "Path to openSeaChest_SMART binary"
    )]
    opensea_smart_bin: String,

    #[arg(
        long = "listen",
        env = "OPENSEA_SMART_LISTEN",
        value_parser = clap::value_parser!(SocketAddr),
        default_value = "0.0.0.0:10988",
        help = "IPv4/6 socket+port address for HTTP listener"
    )]
    listen: SocketAddr,

    #[arg(
        long = "log-level",
        env = "OPENSEA_SMART_LOG_LEVEL",
        value_parser = ["trace", "debug", "info", "warn", "error"],
        default_value = "error",
        help = "Log level for stderr output"
    )]
    log_level: String,

    #[arg(
        long = "host-name",
        env = "OPENSEA_SMART_HOST",
        help = "Hostname for metrics",
        default_value = "host_name_not_set"
    )]
    host_name: String,

    // #[arg(
    //     long = "map-file",
    //     env = "OPENSEA_SMART_MAP_FILE",
    //     help = "JSON file for renaming metrics"
    // )]
    // map_file: Option<String>,
}

async fn gen_metrics(
    _req: Request<Body>,
    options: CliOptions,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let scan_output = Command::new(options.opensea_smart_bin.clone())
        .args(["--scan", "--onlySeagate"])
        .stdout(Stdio::piped())
        .output();
    match scan_output {
        Ok(output) => {
            match parse_smart_scan(output.stdout) {
                Ok(devices) => {
                    let mut attrs_by_type: HashMap<String, Vec<SmartAttribute>> = HashMap::new();
                    let mut devices_by_handle: HashMap<String, SmartDeviceInfo> = HashMap::new();
                    for device in devices {
                        devices_by_handle.insert(device.handle.clone(), device.clone());
                        let flag = format!("-d {} --smartAttributes raw", device.handle.clone());
                        let smart_output = Command::new(options.opensea_smart_bin.clone())
                            .args(flag.split(' '))
                            .stdout(Stdio::piped())
                            .output();
                        match smart_output {
                            Ok(output) => {
                                match parse_smart_attributes(output.stdout, &device) {
                                    Ok(smart_attrs) => {
                                        for attr in smart_attrs {
                                            attrs_by_type
                                                .entry(attr.get_metric_name())
                                                .or_insert(Vec::new())
                                                .push(attr.clone());
                                        }
                                    }
                                    Err(err) => {
                                        error!("Error: {}", err);
                                        return Ok(format!("# ERROR: {}", err).into());
                                    }
                                }
                            },
                            Err(err) => {
                                error!("Error: {}", err);
                                return Ok(format!("# ERROR: {}", err).into());
                            }
                        }
                    }
                    let mut result: String = "".to_owned();
                    for (name, attr_vec) in &attrs_by_type {
                        let first = attr_vec.first().unwrap();
                        let metric_type: MetricType = if first.event_count { MetricType::Counter } else { MetricType::Gauge };
                        let mut pc = PrometheusMetric::build()
                            .with_name(name)
                            .with_metric_type(metric_type)
                            .with_help(first.name.as_str()).build();
                        for attr in attr_vec {
                            let currently_failing: i32 = match attr.currently_failing { Some(f) => {if f {1} else {0}}, None => -1 };
                            let previously_failed: i32 = match attr.previously_failed { Some(f) => {if f {1} else {0}}, None => -1 };

                            pc.render_and_append_instance(
                                &PrometheusInstance::<u8, MissingValue>::new()
                                .with_label("currently_failing", currently_failing.to_string().as_ref())
                                .with_label("previously_failed", previously_failed.to_string().as_ref())
                                .with_label("worst_normalized_value", attr.worst_normalized_value.to_string().as_ref())
                                .with_label("threshold_normalized_value", attr.threshold_normalized_value.to_string().as_ref())
                                .with_label("warranty", attr.warranty.to_string().as_ref())
                                .with_label("decrease_means_degrade", attr.decrease_means_degrade.to_string().as_ref())
                                .with_label("error_rate", attr.error_rate.to_string().as_ref())
                                .with_label("vendor", attr.vendor.as_ref())
                                .with_label("handle", attr.handle.as_ref())
                                .with_label("model_number", attr.model_number.as_ref())
                                .with_label("serial_number", attr.serial_number.as_ref())
                                .with_label("firmware_revision", attr.firmware_revision.as_ref())
                                .with_label("host", options.host_name.as_ref())
                                .with_label("raw", attr.raw_value.to_string().as_ref())
                                .with_label("raw_2b", (attr.raw_value & 0xFFFF).to_string().as_ref())
                                .with_value(attr.normalized_value.into())
                                .with_current_timestamp()
                                .expect("error getting the current UNIX epoch"),
                            );
                        }
                        result.push_str(pc.render().as_str());
                    }
                    return Ok(result.into());
                },
                Err(err) => {
                    error!("Error: {}", err);
                    return Ok(format!("# ERROR: {}", err).into());
                }
            }
        }
        Err(err) => {
            error!("Error: {}", err);
            return Ok(format!("# ERROR: {}", err).into());
        }
    }
    
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli_options = CliOptions::parse();

    env::set_var("RUST_LOG", cli_options.log_level.clone());

    env_logger::init();

    info!(
        "{} v{} starting...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("starting exporter on http://{}/metrics", cli_options.listen);

    let server_options = ServerOptions {
        addr: cli_options.listen,
        authorization: Authorization::None,
    };

    render_prometheus(server_options, cli_options.clone(), |request, _options| {
        Box::pin(gen_metrics(request, cli_options))
    })
    .await;
}
