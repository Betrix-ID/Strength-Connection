#!/system/bin/sh
# Checking ID shell
if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
   exit 1
fi
#Chking cpu.abi
     if [ ! -f /sdcard/native/sys/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
		cp /sdcard/native/sys/target/release/arm64 /sdcard/native/network
	elif [ "$architecture" = "armeabi-v7a" ]; then
		cp /sdcard/native/sys/target/release/arm64 /sdcard/native/network
	fi
  fi
#smart notifications
shell() {
    sor="$1"
    cmd notification post -S bigtext -t '♨️ Strength Connection' 'Tag' "$sor" > /dev/null 2>&1
}
	
# Style display Terminal
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "       ~ Description. Strength Connection...... "
    echo
    echo "       - Author                 :  @UnixeID"
    echo "       - Point                    :  2.0 "
    echo "       - Release               :  29 - Mei - 2025"
    echo "       - Name Shell         :  Strength Connection"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "   Priority Strength Connection Custem. "
    echo
    sleep 2
     rm -rf /data/local/tmp/network
     cp /sdcard/native/network /data/local/tmp
     chmod +x /data/local/tmp/network
     if [ "$1" = "-U" ]; then
          shell "Applying conector use 5ms 1-2 seconds..."
          /data/local/tmp/network -U
     elif [ "$1" = "-S" ]; then
          shell "Applying conector use 30ms 1-2 seconds..."
          /data/local/tmp/network -S
     elif [ "$1" = "-L" ]; then
          shell "Applying conector use 50ms 1-2 seconds..."
          /data/local/tmp/network -L
     elif [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
           /data/local/tmp/network --help
        else
          printf "Failed to apply requested profile. Unknown option: %s\n" "$1"
         fi
set +x

