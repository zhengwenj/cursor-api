/**
 * 更新版本号函数
 * 此函数会读取 VERSION 文件中的数字，将其加1，然后保存回文件
 * 如果 VERSION 文件不存在或为空，将从1开始计数
 * 只在 release 模式下执行，debug/dev 模式下完全跳过
 */
#[cfg(not(debug_assertions))]
#[cfg(feature = "__preview")]
fn update_version() -> Result<()> {
    let version_path = "VERSION";
    // VERSION文件的监控已经在main函数中添加，此处无需重复

    // 读取当前版本号
    let mut version = String::new();
    let mut file = match File::open(version_path) {
        Ok(file) => file,
        Err(_) => {
            // 如果文件不存在或无法打开，从1开始
            println!("cargo:warning=VERSION file not found, creating with initial value 1");
            let mut new_file = File::create(version_path)?;
            new_file.write_all(b"1")?;
            return Ok(());
        }
    };

    file.read_to_string(&mut version)?;

    // 确保版本号是有效数字
    let version_num = match version.trim().parse::<u64>() {
        Ok(num) => num,
        Err(_) => {
            println!("cargo:warning=Invalid version number in VERSION file. Setting to 1.");
            let mut file = File::create(version_path)?;
            file.write_all(b"1")?;
            return Ok(());
        }
    };

    // 版本号加1
    let new_version = version_num + 1;
    println!(
        "cargo:warning=Release build - bumping version from {version_num} to {new_version}",
    );

    // 写回文件
    let mut file = File::create(version_path)?;
    file.write_all(new_version.to_string().as_bytes())?;

    Ok(())
}

#[cfg(feature = "__preview")]
fn read_version_number() -> Result<u64> {
    let mut version = String::with_capacity(4);
    match std::fs::File::open("VERSION") {
        Ok(mut file) => {
            use std::io::Read as _;
            file.read_to_string(&mut version)?;
            Ok(version.trim().parse().unwrap_or(1))
        }
        Err(_) => Ok(1),
    }
}

fn generate_build_info() -> Result<()> {
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(out_dir).join("build_info.rs");
    #[cfg(debug_assertions)]
    let out_dir = "target/debug/build/build_info.rs";
    #[cfg(not(debug_assertions))]
    let out_dir = "target/release/build/build_info.rs";
    let dest_path = Path::new(out_dir);
    // if dest_path.is_file() {
    //     return Ok(());
    // }

    let build_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let build_timestamp_str = chrono::DateTime::from_timestamp(build_timestamp as i64, 0)
        .unwrap()
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    let pkg_version = env!("CARGO_PKG_VERSION");

    #[cfg(feature = "__preview")]
    let (version_str, build_version_str) = {
        let build_num = read_version_number()?;
        (
            format!("{pkg_version}+build.{build_num}"),
            format!("pub const BUILD_VERSION: u32 = {build_num};\n"),
        )
    };

    #[cfg(not(feature = "__preview"))]
    let (version_str, build_version_str) = (pkg_version, "");

    let build_info_content = format!(
        r#"// 此文件由 build.rs 自动生成，请勿手动修改

{build_version_str}pub const BUILD_TIMESTAMP: &'static str = {build_timestamp_str:?};
pub const VERSION: &'static str = {version_str:?};
pub const IS_PRERELEASE: bool = {is_prerelease};
pub const IS_DEBUG: bool = {is_debug};

#[cfg(unix)]
pub const BUILD_EPOCH: std::time::SystemTime = unsafe {{
    #[allow(dead_code)]
    struct UnixSystemTime {{
        tv_sec: i64,
        tv_nsec: u32,
    }}

    ::core::mem::transmute(UnixSystemTime {{
        tv_sec: {build_timestamp},
        tv_nsec: 0,
    }})
}};

#[cfg(windows)]
pub const BUILD_EPOCH: std::time::SystemTime = unsafe {{
    #[allow(dead_code)]
    struct WindowsFileTime {{
        dw_low_date_time: u32,
        dw_high_date_time: u32,
    }}

    const INTERVALS_PER_SEC: u64 = 10_000_000;
    const INTERVALS_TO_UNIX_EPOCH: u64 = 11_644_473_600 * INTERVALS_PER_SEC;
    const TARGET_INTERVALS: u64 = INTERVALS_TO_UNIX_EPOCH + {build_timestamp} * INTERVALS_PER_SEC;

    ::core::mem::transmute(WindowsFileTime {{
        dw_low_date_time: TARGET_INTERVALS as u32,
        dw_high_date_time: (TARGET_INTERVALS >> 32) as u32,
    }})
}};
"#,
        is_prerelease = cfg!(feature = "__preview"),
        is_debug = cfg!(debug_assertions),
    );

    std::fs::write(dest_path, build_info_content)?;
    Ok(())
}
