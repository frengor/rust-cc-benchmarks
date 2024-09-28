#!/usr/bin/env bash

# Benchmarks to execute
benches='stress-test binary-trees binary-trees-with-parent-pointers large-linked-list'

# Garbage collectors to test
gcs='rc arc rust-cc rust-cc-finalization gc bacon-rajan-cc safe-gc broom'

# Benchmark results file
output_file='./bench-results.txt'

# Output format (either 'criterion' or 'bencher')
output_format='bencher'

##########################################################################################

if ! command -v cset >/dev/null; then
  echo "cpuset not found."
  exit 1
fi

if [ ! -f /sys/devices/system/cpu/cpufreq/boost ]; then
  echo "Frequency boosting is not supported."
  exit 1
fi

if [ ! -d /sys/devices/system/cpu/cpu1 ]; then
  echo "CPU1 not found."
  exit 1
fi

boost=$(cat /sys/devices/system/cpu/cpufreq/boost) || exit 1
scaling_governor=$(cat /sys/devices/system/cpu/cpu1/cpufreq/scaling_governor) || exit 1
other_cpus=$(awk -F ',' '{ for (i = 0; ++i <= NF;) if ($i != 1) print $i }' /sys/devices/system/cpu/cpu1/topology/core_cpus_list) || exit 1

sudo cset shield -c 1 -k on || exit 1

echo "Disabling frequency boosting"
echo 0 | sudo tee /sys/devices/system/cpu/cpufreq/boost > /dev/null

echo "Setting scaling governor to performance"
echo performance | sudo tee /sys/devices/system/cpu/cpu1/cpufreq/scaling_governor > /dev/null

for cpu in $other_cpus; do
  echo "Disabling cpu${cpu}"
  echo 0 | sudo tee "/sys/devices/system/cpu/cpu${cpu}/online" > /dev/null
done

sleep 1

# Truncate the output file
true > "${output_file}"

for bench in $benches; do
  for gc in $gcs; do
    if [[ "${bench}" = "stress-test" && ( "${gc}" = "rc" || "${gc}" = "arc" ) ]]; then
      continue
    fi
    echo "Executing benchmark ${bench} for ${gc}"
    cargo bench -F "${bench},${gc}" --no-run || exit 1
    sudo cset shield --exec -- nice -n -20 sudo -u goro bash -l -c "setarch -R cargo bench --no-default-features -F ${bench},${gc} --quiet -- --quiet --color never --output-format ${output_format} >> ${output_file}"
  done
done

for cpu in $other_cpus; do
  echo "Enabling cpu${cpu}"
  echo 1 | sudo tee "/sys/devices/system/cpu/cpu${cpu}/online" > /dev/null
done

echo "Resetting frequency boosting"
echo "${boost}" | sudo tee /sys/devices/system/cpu/cpufreq/boost > /dev/null

echo "Resetting scaling governor to ${scaling_governor}"
echo "${scaling_governor}" | sudo tee /sys/devices/system/cpu/cpu1/cpufreq/scaling_governor > /dev/null

sudo cset shield --reset
