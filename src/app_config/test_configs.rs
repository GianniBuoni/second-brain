/// Test case that sets all optional fields for
/// for `Periodical::Day` and leave the rest as
/// default.
pub const CASE_FULL: &str = "[vault]
dir = \"./vaults\"

[periodical.day]
dir = \"day\"
fmt = \"%Y-%m-%d\"
template = \"day.md\"";

/// Test case that sets mixed optional fields for
/// the periodical nodes.
/// `Periodical::Month` is left as default.
pub const CASE_OPTIONS: &str = "[vault]
dir = \"./vaults\"

[periodical.day]
dir = \"day\"
fmt = \"%m-%d-%Y\"

[periodical.week]
dir = \"period/week\"

[periodical.year]
fmt = \"%Y\"
template = \"/templates/year.md\"";

/// Test case sets only the required vault configuration
/// Everything else should be a default
pub const CASE_DEFAULTS: &str = "[vault]
dir = \"./vaults\"";

/// Case where the configured vault dir is invalid
/// This is a vaild Toml configuration, still.
pub const CASE_INVALID_VAULT: &str = "[vault]
dir = \"invalid\"
    
[periodical.month]
dir = \"period/month\"

[periodical.year]
fmt = \"%Y\"
template = \"year.md\"";
