
pub fn analyze_behavior(payload: &str) -> bool {
    let indicators = vec![
        "inject", "obfuscate", "allocate_ex", "shellcode", "xor_loop", "fork_bomb"
    ];
    indicators.iter().any(|sig| payload.contains(sig))
}
