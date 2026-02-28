//! Module that uses the test case modules to test AppConfig methods
//! and TomlConfig deseriazlizations.
//! Test have to manually construct test AppConfig cases to maintain
//! functional purity, as conversion between TomlConfig -> AppConfig
//! performs validation using system state.
use chrono::Local;

use super::{test_cases::*, *};

#[test]
fn test_parent_dir() -> anyhow::Result<()> {
    let test_cases = [
        ("./vaults", "Test unconfigured sub-directories."),
        ("./vaults/period/week", "Test configured subdirectory"),
    ];
    test_cases.iter().try_for_each(|(want, desc)| {
        let want = PathBuf::from(want);
        let config = AppConfig {
            vault: want.clone(),
            periodical: HashMap::new(),
        };
        let got = config.get_vault_root();
        assert_eq!(want, got, "{desc}");
        anyhow::Ok(())
    })
}

#[test]
fn test_absoulute_note_path() -> anyhow::Result<()> {
    let test_cases = [
        (
            PERIODIC_CASE_DEFAULT,
            Periodical::Day,
            DEFAULT_DAY,
            "Test unconfigured file names",
        ),
        (
            PERIODIC_CASE_OPTIONS,
            Periodical::Day,
            "%m-%d-%Y",
            "Test configured file name.",
        ),
    ];
    test_cases.iter().try_for_each(|(s, period, want, desc)| {
        let config = AppConfig {
            vault: "./vaults".into(),
            periodical: toml::de::from_str::<TomlPeriod>(s)?.0,
        };
        let got = config.try_format_absolute_note_path(*period)?;
        let file_name = Local::now().format(want).to_string();

        assert!(got.to_string_lossy().contains(&file_name), "{desc}");
        assert!(got.to_string_lossy().contains("vaults"), "{desc}");
        anyhow::Ok(())
    })
}

#[test]
fn test_absolute_template_path() -> anyhow::Result<()> {
    let test_cases = [
        (
            PERIODIC_CASE_DEFAULT,
            Periodical::Week,
            None,
            "Case default: test unconfigured template file",
        ),
        (
            PERIODIC_CASE_OPTIONS,
            Periodical::Year,
            Some("templates/year.md"),
            "Case options: test configured template file",
        ),
        (
            PERIODIC_CASE_FULL,
            Periodical::Day,
            Some("day.md"),
            "Case full: test configured template file",
        ),
    ];
    test_cases.iter().try_for_each(|(s, period, want, desc)| {
        let config = AppConfig {
            vault: "./vaults".into(),
            periodical: toml::de::from_str::<TomlPeriod>(s)?.0,
        };
        let got = config.try_format_absolute_template_path(*period)?;
        dbg!(&got);

        match got {
            None => assert!(want.is_none(), "Expeted None: {desc}"),
            Some(path) => {
                assert!(
                    path.to_string_lossy().contains(want.unwrap()),
                    "Check want: {desc}"
                );
                assert!(
                    path.to_string_lossy().contains("vaults"),
                    "Check aubsolute-ish: {desc}"
                );
            }
        }
        anyhow::Ok(())
    })
}

#[test]
/// Test the required vault vaules of configuration cases
/// Tests do not check for configuration validity, only the Toml deserialization.
fn test_de_vault() -> anyhow::Result<()> {
    let test_cases = [
        (
            CASE_DEFAULTS,
            "./vaults",
            "Case default: Test leaving periodcal as default.",
        ),
        (
            CASE_FULL,
            "./vaults",
            "Case full: Test configuring all optional fields",
        ),
        (
            CASE_OPTIONS,
            "./vaults",
            "Case options: Test configuring a mixed bag of settings",
        ),
        (
            CASE_INVALID_VAULT,
            "invalid",
            "Case invalid: Test app configuration that should still sucessfully deserialize",
        ),
    ];
    test_cases.iter().try_for_each(|(s, want, desc)| {
        let got = toml::from_str::<TomlConfig>(s)?;

        assert_eq!(got.vault.dir, PathBuf::from(want), "{desc}");
        anyhow::Ok(())
    })
}

#[test]
fn test_de_period_dir() -> anyhow::Result<()> {
    let period = Periodical::Week;
    let test_cases = [
        (
            CASE_DEFAULTS,
            None,
            "Case default: Test unconfigured parent dir",
        ),
        (
            CASE_OPTIONS,
            Some("period/week".to_string()),
            "Case options: Test configured parent dir",
        ),
    ];
    test_cases.iter().try_for_each(|(s, want, desc)| {
        let got = toml::de::from_str::<TomlConfig>(s)?;
        let got = got.periodical.unwrap_or_default().0;
        let got = got
            .get(&period)
            .unwrap_or(&PeriodConfig::default())
            .get_parent_dir()
            .map(|f| f.to_string());
        assert_eq!(*want, got, "{desc}");
        anyhow::Ok(())
    })
}

#[test]
fn test_de_template_dir() -> anyhow::Result<()> {
    let period = Periodical::Year;
    let test_cases = [
        (
            CASE_OPTIONS,
            Some("templates/year.md".into()),
            "Case options: Test set template config",
        ),
        (CASE_DEFAULTS, None, "Test unset template config"),
        (
            CASE_INVALID_VAULT,
            Some("year.md".into()),
            "Case invalid: Test invalid AppConfig, but valid TomlConfig",
        ),
    ];
    test_cases.iter().try_for_each(|(s, want, desc)| {
        let got = toml::de::from_str::<TomlConfig>(s)?
            .periodical
            .unwrap_or_default()
            .0;
        let got = got
            .get(&period)
            .unwrap_or(&PeriodConfig::default())
            .get_template_file()
            .map(|f| f.to_string());
        assert_eq!(*want, got, "{desc}");
        anyhow::Ok(())
    })
}

#[test]
fn test_de_filename() -> anyhow::Result<()> {
    let period = Periodical::Day;
    let test_cases = [
        (
            CASE_DEFAULTS,
            Local::now().format(DEFAULT_DAY),
            "Case default: Test unset filename formatting",
        ),
        (
            CASE_OPTIONS,
            Local::now().format("%m-%d-%Y"),
            "Case options: Test changed filename formatting",
        ),
        (
            CASE_INVALID_VAULT,
            Local::now().format(DEFAULT_DAY),
            "Case invalid: Test invalid AppConfig, but valid TomlConfig",
        ),
    ];
    test_cases.iter().try_for_each(|(s, want, desc)| {
        let got = toml::de::from_str::<TomlConfig>(s)?
            .periodical
            .unwrap_or_default()
            .0;
        let got = got
            .get(&period)
            .unwrap_or(&PeriodConfig::default())
            .format_file_name(period);
        assert_eq!(format!("{want}.md"), got, "{desc}");
        anyhow::Ok(())
    })
}

#[test]
fn test_invalid_vault() -> anyhow::Result<()> {
    let desc =
        "Test invalid/non-existant vault directory in the TomlConfig to AppConfig conversion.";
    let want = ConfigError::InvalidDir("invalid".into());

    let s = CASE_INVALID_VAULT.as_bytes();
    let got = toml::de::from_slice::<TomlConfig>(s)?;
    let got = AppConfig::try_from(got);

    match got {
        Ok(e) => panic!("Expected error, got {e:?}."),
        Err(e) => assert_eq!(want.to_string(), e.to_string(), "{desc}"),
    }
    Ok(())
}
