pub fn usage() {
    let script_version = "1.0.1";
    println!(r#"♨️ Strength Connection {} - Android Network Tweaking Utility

Usage:
  NetOptimizer [OPTION]

Options:
  -S        Apply balanced network optimization with 60ms screen-on delay.
  -U        Apply moderate network delay profile with 80ms screen-on delay.
  -L        Apply conservative delay profile with 125ms screen-on delay.
  -d        Reset latency wifi and network to defatult settings
  -h, --help     Show this help message and exit.

Description:
  NetOptimizer is a command-line tool designed to tune Android's wireless and network-related
  system properties. It modifies parameters related to Wi-Fi scoring, watchdog behavior, DNS
  resolver, and connectivity performance to improve overall network responsiveness and stability.

  Each profile adjusts the screen-on delay and related properties for different performance needs:
    - netd60: Best for low-latency, responsive wake performance.
    - netd80: Balanced between responsiveness and network readiness.
    - netd125: Ideal for older or slower devices requiring longer delays.
    - default: reset configuration latency proider 

Examples:
  Apply fast network optimization:
      NetOptimizer  -S

  Apply balanced optimization:
      NetOptimizer  -U

  Apply stable profile for legacy devices:
      NetOptimizer  -L
      
  default latency profile for legacy device:
      defaultsettings -d

  Show help:
      NetOptimizer   -h --help

Requirements:
  - Root access is required to apply system property changes.
  - Device must support adb shell with access to 'setprop' and 'cmd' utilities.

More info:
  - Wi-Fi scoring: https://cs.android.com/search?q=wifi_score_params
  - Connectivity properties: https://cs.android.com/search?q=connectivity_metrics_buffer_size
  - Screen-on delay setting: https://cs.android.com/search?q=set-one-shot-screen-on-delay-ms
"#,
    script_version);
}