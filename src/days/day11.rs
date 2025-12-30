use aoc::Context;
use std::collections::HashMap;

pub fn solve(ctx: &mut Context) {
    let mut devices = HashMap::new();
    for line in ctx.input() {
        let parts: Vec<&str> = line.split(':').collect();
        let device = parts[0].trim();
        let connections: Vec<String> = parts[1].trim().split(' ').map(|s| s.to_string()).collect();
        devices.insert(device.to_string(), connections);
    }

    // Note: examples for part 1 and 2 are different.
    // So the same run will not work for both parts with example input.

    // Part one: paths from "you" to "out" (no required nodes)
    let result = count_paths(&devices, "you", "out", false, false);
    ctx.set_sol1(result);

    // Part two: paths from "svr" to "out" that pass through both "dac" and "fft"
    let result2 = count_paths(&devices, "svr", "out", true, true);
    ctx.set_sol2(result2);
}

struct DfsParams {
    require_dac: bool,
    require_fft: bool,
}

fn count_paths(
    devices: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    require_dac: bool,
    require_fft: bool,
) -> usize {
    let mut cache = HashMap::new();
    let params = DfsParams {
        require_dac,
        require_fft,
    };
    dfs_with_flags(devices, start, end, false, false, &params, &mut cache)
}

fn dfs_with_flags(
    devices: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    seen_dac: bool,
    seen_fft: bool,
    params: &DfsParams,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    // Update flags based on current node
    let seen_dac = seen_dac || current == "dac";
    let seen_fft = seen_fft || current == "fft";

    // Base case: reached the end
    if current == end {
        let dac_ok = !params.require_dac || seen_dac;
        let fft_ok = !params.require_fft || seen_fft;
        return if dac_ok && fft_ok { 1 } else { 0 };
    }

    // Check cache - must include flags since they affect the result
    let cache_key = (current.to_string(), seen_dac, seen_fft);
    if let Some(&result) = cache.get(&cache_key) {
        return result;
    }

    // Recursive case: sum paths through all neighbors
    let mut total_paths = 0;
    if let Some(neighbors) = devices.get(current) {
        for neighbor in neighbors {
            total_paths +=
                dfs_with_flags(devices, neighbor, end, seen_dac, seen_fft, params, cache);
        }
    }

    // Cache and return result
    cache.insert(cache_key, total_paths);
    total_paths
}
