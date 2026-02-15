# Simple example

## Notes
This has to be self contained because for w/e reason OUT_DIR isnt set in `cargo run --example <>`, /shrug

## Expected output
```rs
Some(
    Ok(
        BuildInfo {
            package: Package {
                name: Some(
                    "simple",
                ),
                version: Some(
                    "0.1.0",
                ),
                authors: Some(
                    "",
                ),
                edition: Some(
                    "2024",
                ),
                msrv: None,
                description: None,
                documentation: None,
                readme: None,
                homepage: None,
                repository: None,
                license: None,
                license_file: None,
                keywords: Some(
                    "",
                ),
                categories: Some(
                    "",
                ),
                links: None,
                publish: None,
                metadata: None,
                default_run: None,
            },
            agent: Agent {
                hostname: "somehostname",
                os: AgentOs {
                    distribution: "macos",
                    distribution_like: "",
                    name: "Darwin",
                    version: "25.0.0",
                    long_version: "macOS 26.0",
                    kernel: Some(
                        "26.0",
                    ),
                    architecture: "arm64",
                },
                memory: AgentMemory {
                    total: ByteAmount {
                        bytes: 25769803776,
                        human: "25.77 GB",
                    },
                    used: ByteAmount {
                        bytes: 15923658752,
                        human: "15.92 GB",
                    },
                    free: ByteAmount {
                        bytes: 642170880,
                        human: "642.17 MB",
                    },
                },
                swap: AgentMemory {
                    total: ByteAmount {
                        bytes: 1073741824,
                        human: "1.07 GB",
                    },
                    used: ByteAmount {
                        bytes: 229703680,
                        human: "229.70 MB",
                    },
                    free: ByteAmount {
                        bytes: 844038144,
                        human: "844.04 MB",
                    },
                },
                ncpus: 14,
            },
        },
    )
)
```
