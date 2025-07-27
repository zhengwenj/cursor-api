//! 项目和构建时间追踪模块
//!
//! 提供跨平台的时间基准点定义和优雅的时间差显示功能。
//! 支持项目启动时间（按天计算）和构建时间（按分钟计算）的追踪。

use std::fmt;

pub use super::build::BUILD_EPOCH;

/// Unix 系统的时间基准点（2024-12-23 01:30:48 UTC）
#[cfg(unix)]
pub const EPOCH: std::time::SystemTime = unsafe {
    #[repr(C)]
    struct UnixSystemTime {
        tv_sec: i64,
        tv_nsec: u32,
    }

    ::core::intrinsics::transmute_unchecked(UnixSystemTime {
        tv_sec: 1734915448,
        tv_nsec: 0,
    })
};

/// Windows 系统的时间基准点（2024-12-23 01:30:48 UTC）
#[cfg(windows)]
pub const EPOCH: std::time::SystemTime = unsafe {
    #[repr(C)]
    struct WindowsFileTime {
        dw_low_date_time: u32,
        dw_high_date_time: u32,
    }

    const INTERVALS_PER_SEC: u64 = 10_000_000;
    const INTERVALS_TO_UNIX_EPOCH: u64 = 11_644_473_600 * INTERVALS_PER_SEC;
    const TARGET_INTERVALS: u64 = INTERVALS_TO_UNIX_EPOCH + 1734915448 * INTERVALS_PER_SEC;

    ::core::intrinsics::transmute_unchecked(WindowsFileTime {
        dw_low_date_time: TARGET_INTERVALS as u32,
        dw_high_date_time: (TARGET_INTERVALS >> 32) as u32,
    })
};

/// 打印项目启动以来的时间
///
/// 以年、月、日的形式显示项目运行时长
pub fn print_project_age() {
    let age = ProjectAge::since_epoch();
    println!("Project started {age} ago");
}

/// 打印程序构建以来的时间
///
/// 以分钟级精度显示构建时长，包含友好的时间描述
pub fn print_build_age() {
    let age = BuildAge::since_build();
    println!("Program built {age} ago");
}

/// 项目年龄表示，以天为最小单位
///
/// 用于表示较长时间跨度，适合项目生命周期追踪
#[derive(Debug, Clone, Copy)]
struct ProjectAge {
    years: u64,
    months: u64,
    days: u64,
}

impl ProjectAge {
    /// 计算自项目 EPOCH 以来的时间
    ///
    /// # Panics
    ///
    /// 如果系统时间早于项目 EPOCH 时间则会 panic
    #[inline]
    pub fn since_epoch() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(EPOCH)
            .expect("system time before program epoch");

        Self::from_days(duration.as_secs() / 86400)
    }

    /// 从总天数创建 ProjectAge 实例
    ///
    /// 使用简化的时间计算：
    /// - 1年 = 365天
    /// - 1月 = 30天
    /// - 包含基本的闰年调整
    #[inline]
    fn from_days(total_days: u64) -> Self {
        if total_days == 0 {
            return Self {
                years: 0,
                months: 0,
                days: 0,
            };
        }

        let years = total_days / 365;
        let remaining_days = total_days % 365;

        // 简单的闰年调整
        let leap_adjustment = years / 4;
        let adjusted_days = remaining_days.saturating_sub(leap_adjustment);

        let months = adjusted_days / 30;
        let days = adjusted_days % 30;

        Self {
            years,
            months,
            days,
        }
    }
}

impl fmt::Display for ProjectAge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.years, self.months, self.days) {
            (0, 0, 0) => write!(f, "less than 1 day"),
            (0, 0, d) => write!(f, "{d} day{}", if d == 1 { "" } else { "s" }),
            (0, m, 0) => write!(f, "{m} month{}", if m == 1 { "" } else { "s" }),
            (0, m, d) => write!(
                f,
                "{m} month{} and {d} day{}",
                if m == 1 { "" } else { "s" },
                if d == 1 { "" } else { "s" }
            ),
            (y, 0, 0) => write!(f, "{y} year{}", if y == 1 { "" } else { "s" }),
            (y, 0, d) => write!(
                f,
                "{y} year{} and {d} day{}",
                if y == 1 { "" } else { "s" },
                if d == 1 { "" } else { "s" }
            ),
            (y, m, 0) => write!(
                f,
                "{y} year{} and {m} month{}",
                if y == 1 { "" } else { "s" },
                if m == 1 { "" } else { "s" }
            ),
            (y, m, d) => write!(
                f,
                "{y} year{}, {m} month{}, and {d} day{}",
                if y == 1 { "" } else { "s" },
                if m == 1 { "" } else { "s" },
                if d == 1 { "" } else { "s" }
            ),
        }
    }
}

/// 构建年龄表示，以分钟为最小单位
///
/// 用于表示较短时间跨度，适合构建时间追踪，提供友好的时间描述
#[derive(Debug, Clone, Copy)]
struct BuildAge {
    minutes: u64,
}

impl BuildAge {
    /// 计算自构建 EPOCH 以来的时间
    ///
    /// # Panics
    ///
    /// 如果系统时间早于构建 EPOCH 时间则会 panic
    #[inline]
    pub fn since_build() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(BUILD_EPOCH)
            .expect("system time before build epoch");

        Self {
            minutes: duration.as_secs() / 60,
        }
    }
}

impl fmt::Display for BuildAge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.minutes {
            0 => write!(f, "just now"),
            1 => write!(f, "1 minute"),
            m if m < 60 => write!(f, "{m} minutes"),
            m if m < 120 => {
                let mins = m % 60;
                if mins == 0 {
                    write!(f, "1 hour")
                } else {
                    write!(
                        f,
                        "1 hour and {mins} minute{}",
                        if mins == 1 { "" } else { "s" }
                    )
                }
            }
            m if m < 1440 => {
                let hours = m / 60;
                let mins = m % 60;
                if mins == 0 {
                    write!(f, "{hours} hour{}", if hours == 1 { "" } else { "s" })
                } else {
                    write!(
                        f,
                        "{hours} hour{} and {mins} minute{}",
                        if hours == 1 { "" } else { "s" },
                        if mins == 1 { "" } else { "s" }
                    )
                }
            }
            m => {
                let days = m / 1440;
                let remaining_hours = (m % 1440) / 60;
                if remaining_hours == 0 {
                    write!(f, "{days} day{}", if days == 1 { "" } else { "s" })
                } else {
                    write!(
                        f,
                        "{days} day{} and {remaining_hours} hour{}",
                        if days == 1 { "" } else { "s" },
                        if remaining_hours == 1 { "" } else { "s" }
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_age_display() {
        let age = ProjectAge {
            years: 0,
            months: 0,
            days: 0,
        };
        assert_eq!(format!("{}", age), "less than 1 day");

        let age = ProjectAge {
            years: 1,
            months: 2,
            days: 3,
        };
        assert_eq!(format!("{}", age), "1 year, 2 months, and 3 days");

        let age = ProjectAge {
            years: 2,
            months: 1,
            days: 1,
        };
        assert_eq!(format!("{}", age), "2 years, 1 month, and 1 day");
    }

    #[test]
    fn test_build_age_display() {
        let age = BuildAge { minutes: 0 };
        assert_eq!(format!("{}", age), "just now");

        let age = BuildAge { minutes: 1 };
        assert_eq!(format!("{}", age), "1 minute");

        let age = BuildAge { minutes: 65 };
        assert_eq!(format!("{}", age), "1 hour and 5 minutes");

        let age = BuildAge { minutes: 120 };
        assert_eq!(format!("{}", age), "2 hours");
    }
}
