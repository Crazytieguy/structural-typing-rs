//! # Merging Patterns Example
//!
//! This example demonstrates various patterns for merging partial data.
//! Shows how data from different sources can be combined progressively.

use structural_typing::structural;

#[structural]
#[derive(Debug, Clone, PartialEq)]
struct Config {
    #[always]
    app_name: String,

    host: String,
    port: u16,
    database_url: String,
    cache_size: usize,
    log_level: String,
    feature_flags: String,
}

#[structural]
#[derive(Debug, Clone)]
struct UserPreferences {
    #[always]
    user_id: i64,

    theme: String,
    language: String,
    timezone: String,
    notifications_enabled: bool,
    email_frequency: String,
}

fn main() {
    println!("=== Merging Patterns Example ===\n");

    println!("1. Basic merge - combining partial configurations:");
    let defaults = Config::empty("MyApp".into())
        .host("localhost".into())
        .port(8080);

    let overrides = Config::empty("MyApp".into())
        .port(3000)
        .database_url("postgres://localhost/mydb".into());

    let merged = defaults.merge(overrides);
    println!("   Defaults: host=localhost, port=8080");
    println!("   Overrides: port=3000, database_url=postgres://...");
    println!("   Result: host={}, port={}", merged.host, merged.port);
    println!("   ✓ Override takes precedence, defaults preserved");

    println!("\n2. Multiple source merging (CLI > Env > Config File > Defaults):");

    let defaults = Config::empty("MyApp".into())
        .host("localhost".into())
        .port(8080)
        .cache_size(1024)
        .log_level("info".into());

    let config_file = Config::empty("MyApp".into())
        .host("prod.example.com".into())
        .database_url("postgres://prod/db".into());

    let env_vars = Config::empty("MyApp".into())
        .port(9000)
        .log_level("debug".into());

    let cli_args = Config::empty("MyApp".into())
        .port(3000);

    let final_config = defaults
        .merge(config_file)
        .merge(env_vars)
        .merge(cli_args);

    println!("   Sources:");
    println!("     - Defaults: host=localhost, port=8080, cache=1024, log=info");
    println!("     - Config file: host=prod.example.com, database=postgres://...");
    println!("     - Environment: port=9000, log=debug");
    println!("     - CLI args: port=3000");
    println!("\n   Final configuration:");
    println!("     Host: {} (from config file)", final_config.host);
    println!("     Port: {} (from CLI)", final_config.port);
    println!("     Cache: {} (from defaults)", final_config.cache_size);
    println!("     Log level: {} (from environment)", final_config.log_level);
    println!("     Database: {} (from config file)", final_config.database_url);

    println!("\n3. Merging user preferences over time:");

    let initial_prefs = UserPreferences::empty(1)
        .theme("light".into())
        .language("en".into());
    println!("   Initial: theme=light, language=en");

    let update1 = UserPreferences::empty(1)
        .timezone("America/New_York".into())
        .notifications_enabled(true);
    let prefs_v1 = initial_prefs.merge(update1);
    println!("   After update 1: added timezone and notifications");
    println!("     Theme: {}", prefs_v1.theme);
    println!("     Timezone: {}", prefs_v1.timezone);

    let update2 = UserPreferences::empty(1)
        .theme("dark".into())
        .email_frequency("daily".into());
    let prefs_v2 = prefs_v1.merge(update2);
    println!("   After update 2: changed theme, added email frequency");
    println!("     Theme: {} (updated)", prefs_v2.theme);
    println!("     Language: {} (preserved)", prefs_v2.language);
    println!("     Email frequency: {}", prefs_v2.email_frequency);

    println!("\n4. Merging partial config with defaults:");

    let partial_config = Config::empty("MyApp".into())
        .database_url("postgres://localhost/test".into());

    let defaults = Config::empty("MyApp".into())
        .host("localhost".into())
        .port(8080)
        .cache_size(1024)
        .log_level("info".into())
        .feature_flags("".into());

    let complete_config = defaults.merge(partial_config);

    println!("   Partial config: only database_url set");
    println!("   After merging with defaults:");
    println!("     Host: {} (default)", complete_config.host);
    println!("     Port: {} (default)", complete_config.port);
    println!("     Database: {} (from partial)", complete_config.database_url);
    println!("     Cache: {} (default)", complete_config.cache_size);

    println!("\n5. Merging from multiple users/sources:");

    let _user1_prefs = UserPreferences::empty(1)
        .theme("dark".into())
        .language("en".into())
        .notifications_enabled(true);

    let _user2_prefs = UserPreferences::empty(2)
        .theme("light".into())
        .language("es".into())
        .timezone("Europe/Madrid".into());

    let _user3_prefs = UserPreferences::empty(3)
        .language("fr".into())
        .email_frequency("weekly".into());

    println!("   User 1: theme=dark, language=en, notifications=true");
    println!("   User 2: theme=light, language=es, timezone=Europe/Madrid");
    println!("   User 3: language=fr, email_frequency=weekly");

    let combined = UserPreferences::empty(999)
        .theme("dark".into())
        .language("en".into())
        .notifications_enabled(true)
        .merge(UserPreferences::empty(999).timezone("Europe/Madrid".into()))
        .merge(UserPreferences::empty(999).email_frequency("weekly".into()));

    println!("\n   Combined preferences (picking from all):");
    println!("     Theme: {}", combined.theme);
    println!("     Language: {}", combined.language);
    println!("     Timezone: {}", combined.timezone);
    println!("     Email: {}", combined.email_frequency);

    println!("\n6. Empty merge behavior:");

    let empty_config = Config::empty("MyApp".into());
    let with_data = Config::empty("MyApp".into())
        .host("example.com".into())
        .port(8080);

    let result = empty_config.merge(with_data.clone());
    println!("   Empty merged with data: host={}, port={}", result.host, result.port);

    let result2 = with_data.merge(Config::empty("MyApp".into()));
    println!("   Data merged with empty: host={}, port={}", result2.host, result2.port);
    println!("   ✓ Data preserved in both directions");

    println!("\n=== Key Benefits ===");
    println!("✓ Progressive data construction from multiple sources");
    println!("✓ Later merges override earlier values");
    println!("✓ Type-safe: can't merge incompatible types");
    println!("✓ Always-present fields come from the 'other' (right) side");
    println!("✓ Perfect for config cascading and user preferences");
}
