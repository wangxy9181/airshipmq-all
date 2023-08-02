use anyhow::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    // 将 protobuf 的 bytes 类型生成为 Bytes，而非默认的 Vec<u8>
    config.bytes(&["."]);
    config.out_dir("src/pb")
        .compile_protos(&[
            "abi.proto",
    ], &[
            ".",
        ])?;
    Ok(())
}