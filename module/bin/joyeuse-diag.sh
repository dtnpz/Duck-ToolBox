#!/system/bin/sh
# Read-only device compatibility report. Never opens a TA or provisions a key.
set -u

echo "Duck ToolBox dtnpz / joyeuse diagnostic (read-only)"
echo "No keybox, private keys, CSR or attestation secrets are collected."
echo

property() {
  value="$(getprop "$1" 2>/dev/null)"
  [ -n "$value" ] || value="(unset)"
  printf '%-33s %s\n' "$1" "$value"
}

path_status() {
  if [ -e "$1" ]; then
    printf 'present  %s\n' "$1"
  else
    printf 'missing  %s\n' "$1"
  fi
}

echo "Device properties:"
for name in \
  ro.product.device ro.vendor.product.device \
  ro.build.version.release ro.build.version.sdk \
  ro.soc.manufacturer ro.soc.model ro.vendor.build.security_patch \
  ro.build.fingerprint
do
  property "$name"
done

echo
echo "Hardware and vendor entry points:"
for name in \
  /dev/smcinvoke \
  /vendor/lib64/libkeymasterdeviceutils.so \
  /system/bin/app_process \
  /system/bin/app_process64
do
  path_status "$name"
done

echo
echo "Additional vendor HAL candidates:"
found=0
for name in \
  /vendor/lib64/hw/android.hardware.keymaster@*.so \
  /vendor/lib64/hw/android.hardware.security.keymint*.so
do
  if [ -f "$name" ]; then
    printf 'present  %s\n' "$name"
    found=1
  fi
done
[ "$found" -eq 1 ] || echo "(none found with these names)"

echo
echo "Duck ToolBox prerequisite result:"
wrapper=/data/adb/modules/duck-toolbox/bin/duckctl.sh
if [ -f "$wrapper" ]; then
  if sh "$wrapper" rkp-device status; then
    :
  else
    echo "The backend returned an error; inspect its output above."
  fi
else
  echo "Duck ToolBox is not installed at the expected module path."
fi

echo
echo "A present library or successful prerequisite check does NOT prove"
echo "that firmware, DICE, the TA ABI or Google's RKP server will accept issuance."
