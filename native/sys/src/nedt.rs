/* 
 notes critical user :
 - You may not change, modify or even put this script into your personal project without written permission from the script creator (official author) 
 
 Telegarm    : @UnixeID | Chenel : @Yeye_PID
 Github      : Betrix-ID
 Consultasi  : betrix322@gmail.com
 
 the date this script was written : 11 - Mei - 2025
*/

use std::{
    process::Command,
    thread::sleep,
    time::Duration,
    collections::HashMap,
};


fn line(shell: &str) {
    let messge = format!(
        "cmd notification post -S bigtext -t '♨️ Strength Connection' 'Tag' '{}' > /dev/null 2>&1",
        shell
    );
    let _ = Command::new("sh").arg("-c").arg(messge).status();
}
 
fn props(prop: &str, value: &str) {
      if value.is_empty() {
         let _ = Command::new("sh")
                 .arg("-c")
                 .arg(format!("settings delete global {}", prop))
                 .status();
      } else {
      let cmd = format!("settings put global {} {}",prop, value);
      let output = Command::new("sh")
                   .arg("-c")
                   .arg(&cmd)
                   .output();
                   
         if let Err(e) = output {
            eprintln!("Failed to set {} {}: {}", prop, value, e);
      }
   }
}
 
fn latency(ms: u32) {
   let delay = format!(
        "cmd wifi set-one-shot-screen-on-delay-ms {}"
        ,ms
     );
    let _ = Command::new("sh").arg("-c").arg(delay).status();
 }
    
 
fn frozen(delay: u64) {
  sleep(Duration::from_secs(delay));
 }
 
fn ping_fooling() {
    let mut pinging = HashMap::new();
    pinging.insert("1.1.1.1", "1dot1dot1dot1.cloudflare-dns.com");
    pinging.insert("9.9.9.9", "dns.quad9.net");
    pinging.insert("8.8.8.8", "dns.google");
    pinging.insert("94.140.14.14", "dns.adguard.com");
    pinging.insert("0.0.0.0", "doh.tiar.app");
    
    let mut best_ping = "";
    let mut best_avg = f64::MAX;
    
    for ip in ["1.1.1.1", "9.9.9.9", "8.8.8.8", "94.140.14.14"] {
        println!("\nDNS Profider {} ....", ip);
                
    let cmd = format!(
       "ping -i 0.2 -s 32 -c 10 {} | awk -F 'time=' '/time=/{{n++; sum+=$2}} END {{if (n>0) print \"Avg:\", sum/n, \"ms\"; else print \"No response.\"}}'",
         ip
       );
    
      if let Ok(loping) = Command::new("sh").arg("-c").arg(&cmd).output() {
       let input = String::from_utf8_lossy(&loping.stdout);        
         let trimmed = input.trim();
           if trimmed.contains("Avg") {
             let end: Vec<&str> = trimmed.split_whitespace().collect();
                        if end.len() >= 2 {
              if let Ok(avg) = end[1].parse::<f64>() {
                println!("Average for {}: {:2} ms", ip , avg);
               if avg == 29.0 || avg == 32.0 {
                  println!("Skpping {} due to suspicious avg ping {} ms",ip ,avg);
                  continue;
                 }                
                        if avg < best_avg {
                          best_avg = avg;
                          best_ping = ip;
                     }
                 }
             }
         }
     }
 }
    
   if best_ping.is_empty() {
      println!("\nAl dns tests failed. Using fallback.");
      best_ping = "0.0.0.0";
   }
   
   if let Some(specifier) = pinging.get(best_ping) {
         println!("\nSettings Dns to {} -> {}", best_ping ,specifier);
         props("private_dns_mode", "hostname");
         props("private_dns_specifier", specifier);
         line(&format!("Dns set to {}", best_ping));
     } else {
        println!("No vaild Dns found. Skipping change.");
    }   
}

pub fn cout_5() {       
        println!(
        "\tDescription:
    Applies connectivity and network-related system properties aimed at
    optimizing Wi-Fi and mobile data behavior. This configuration sets
    signal thresholds, disables network watchdogs, and prioritizes certain
    multipath preferences. The screen-on delay is configured to 60ms to
    potentially improve responsiveness on wake."
    );
        
   let command = [
       ("wifi_score_params", "rssi2=-95:-85:-73:-60,rssi5=-85:-82:-70:-57"),
       ("preferred_network_mode_choices_world_mode", "Global"),
       ("connectivity_release_pending_intent_delay_ms", "1800000"),
       ("wifi_mobile_data_transition_wakelock_timeout_ms", "50000"),
       ("preferred_network_mode_values_world_mode", "8"),
       ("dns_resolver_success_threshold_percent", "8"),
       ("wifi_watchdog_poor_network_test_enabled", "0"),
       ("wifi_watchdog_background_check_enabled", "0"),
       ("network_metered_multipath_preference", "2"),
       ("wifi_networks_available_notification_on", "0"),
       ("connectivity_metrics_buffer_size", "250000"),
       ("suspend_optimizations_enabled", "0"),
       ("wifi_max_dhcp_retry_count", "5"),
       ("preferred_network_mode", "9,9"),
       ("wifi_scan_always_enabled", "0"),
       ("network_avoid_bad_wifi", "0"),
       ("wifi_idle_ms", "1800000"),
       ("netstats_enabled", "1"),
       ("network_scoring_ui_enabled", "0"),
       ("wifi_watchdog_on", "0"),
       ("wifi_sleep_policy", "2"), 
   ];
     for (prop, value) in command {
         props(prop, value);
    }
    
    latency(5);
    ping_fooling();
    frozen(1);
    line("Successfully: Apply Latency 60ns....");
}

pub fn cout_30() {       
        println!(
        "\tDescription:
    Similar to netd_60, this profile applies optimized network properties
    with a slightly increased screen-on delay (80ms). Intended for devices
    where a longer wake delay helps stabilize network connectivity upon
    screen activation."
    );
     
   let command = [
       ("wifi_score_params", "rssi2=-95:-85:-73:-60,rssi5=-85:-82:-70:-57"),
       ("preferred_network_mode_choices_world_mode", "Global"),
       ("connectivity_release_pending_intent_delay_ms", "1800000"),
       ("wifi_mobile_data_transition_wakelock_timeout_ms", "50000"),
       ("preferred_network_mode_values_world_mode", "8"),
       ("dns_resolver_success_threshold_percent", "8"),
       ("wifi_watchdog_poor_network_test_enabled", "0"),
       ("wifi_watchdog_background_check_enabled", "0"),
       ("network_metered_multipath_preference", "2"),
       ("wifi_networks_available_notification_on", "0"),
       ("connectivity_metrics_buffer_size", "250000"),
       ("suspend_optimizations_enabled", "0"),
       ("wifi_max_dhcp_retry_count", "5"),
       ("preferred_network_mode", "9,9"),
       ("wifi_scan_always_enabled", "0"),
       ("network_avoid_bad_wifi", "0"),
       ("wifi_idle_ms", "1800000"),
       ("netstats_enabled", "1"),
       ("network_scoring_ui_enabled", "0"),
       ("wifi_watchdog_on", "0"),
       ("wifi_sleep_policy", "2"), 
   ];
     for (prop, value) in command {
         props(prop, value);
    }
    
    latency(30);
    ping_fooling();
    frozen(1);
    line("Successfully: Apply Latency 80ns....");
}

pub fn cout_50() {       
        println!(
        "\tDescription:
    This version applies the same core network optimizations, but configures
    the screen-on delay to 125ms. Best used for older or slower devices that
    require more time to establish a reliable network connection after wake."
    );
     
   let command = [
       ("wifi_score_params", "rssi2=-95:-85:-73:-60,rssi5=-85:-82:-70:-57"),
       ("preferred_network_mode_choices_world_mode", "Global"),
       ("connectivity_release_pending_intent_delay_ms", "1800000"),
       ("wifi_mobile_data_transition_wakelock_timeout_ms", "50000"),
       ("preferred_network_mode_values_world_mode", "8"),
       ("dns_resolver_success_threshold_percent", "8"),
       ("wifi_watchdog_poor_network_test_enabled", "0"),
       ("wifi_watchdog_background_check_enabled", "0"),
       ("network_metered_multipath_preference", "2"),
       ("wifi_networks_available_notification_on", "0"),
       ("connectivity_metrics_buffer_size", "250000"),
       ("suspend_optimizations_enabled", "0"),
       ("wifi_max_dhcp_retry_count", "5"),
       ("preferred_network_mode", "9,9"),
       ("wifi_scan_always_enabled", "0"),
       ("network_avoid_bad_wifi", "0"),
       ("wifi_idle_ms", "1800000"),
       ("netstats_enabled", "1"),
       ("network_scoring_ui_enabled", "0"),
       ("wifi_watchdog_on", "0"),
       ("wifi_sleep_policy", "2"), 
   ];
     for (prop, value) in command {
         props(prop, value);
    }
    
    latency(50);
    ping_fooling();
    frozen(1);
    line("Successfully: Apply Latency 125ns.....");
}

pub fn deafult() {
        println!("\tDescription:
       Restores default connectivity and network-related system properties to 
       their original state as typically configured by the Android system.
       This includes re-enabling watchdogs, resetting signal thresholds, 
       and restoring system-defined timeouts and behaviors. Useful for 
       troubleshooting or returning to baseline performance after tweaks."
          );
          
     let command = [
       ("wifi_score_params", ""), 
       ("preferred_network_mode_choices_world_mode", "Global"),
       ("connectivity_release_pending_intent_delay_ms", "5000"),
       ("wifi_mobile_data_transition_wakelock_timeout_ms", "3000"),
       ("preferred_network_mode_values_world_mode", "0"),
       ("dns_resolver_success_threshold_percent", "25"),
       ("wifi_watchdog_poor_network_test_enabled", "1"),
       ("wifi_watchdog_background_check_enabled", "1"),
       ("network_metered_multipath_preference", "1"),
       ("wifi_networks_available_notification_on", "1"),
       ("connectivity_metrics_buffer_size", "200000"),
       ("suspend_optimizations_enabled", "1"),
       ("wifi_max_dhcp_retry_count", "9"),
       ("preferred_network_mode", "0,0"),
       ("wifi_scan_always_enabled", "1"),
       ("network_avoid_bad_wifi", "1"),
       ("wifi_idle_ms", "300000"), 
       ("netstats_enabled", "1"),
       ("network_scoring_ui_enabled", "1"),
       ("wifi_watchdog_on", "1"),
       ("wifi_sleep_policy", "0"),
   ];
            
   for (prop, value) in command {
         props(prop, value);
    }

    latency(25);
    frozen(1);
    line("Successfully: reset to default settings.....");
}