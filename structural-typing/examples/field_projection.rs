//! # Field Projection Example
//!
//! This example demonstrates selecting/projecting specific fields from a struct.
//! Useful for hiding sensitive data, creating API responses, or display purposes.

use structural_typing::{structural, Present};

#[structural]
#[derive(Debug, Clone)]
struct User {
    #[always]
    id: i64,

    username: String,
    email: String,
    password_hash: String,

    full_name: String,
    bio: String,
    avatar_url: String,

    last_login_ip: String,
    api_key: String,
}

type PublicProfile = User<
    user_state::SetAvatarUrl<
        user_state::SetBio<
            user_state::SetFullName<
                user_state::SetUsername<
                    user_state::Empty
                >
            >
        >
    >
>;

type AdminView = User<
    user_state::SetLastLoginIp<
        user_state::SetEmail<
            user_state::SetUsername<
                user_state::Empty
            >
        >
    >
>;

type BasicInfo = User<
    user_state::SetEmail<
        user_state::SetUsername<
            user_state::Empty
        >
    >
>;

impl FullUser {
    fn to_public_profile(&self) -> PublicProfile {
        User::new(self.id)
            .username(self.username.clone())
            .full_name(self.full_name.clone())
            .bio(self.bio.clone())
            .avatar_url(self.avatar_url.clone())
    }

    fn to_admin_view(&self) -> AdminView {
        User::new(self.id)
            .username(self.username.clone())
            .email(self.email.clone())
            .last_login_ip(self.last_login_ip.clone())
    }

    fn to_basic_info(&self) -> BasicInfo {
        User::new(self.id)
            .username(self.username.clone())
            .email(self.email.clone())
    }

    fn to_json_safe(&self) -> String {
        format!(
            r#"{{"id":{},"username":"{}","email":"{}"}}"#,
            self.id, self.username, self.email
        )
    }
}

impl PublicProfile {
    fn display(&self) -> String {
        let mut parts = vec![
            format!("@{}", self.username),
        ];

        if !self.full_name.is_empty() {
            parts.insert(0, self.full_name.clone());
        }

        if !self.bio.is_empty() {
            parts.push(format!("\n{}", self.bio));
        }

        if !self.avatar_url.is_empty() {
            parts.push(format!("\n🖼️  {}", self.avatar_url));
        }

        parts.join(" ")
    }
}

impl AdminView {
    fn display(&self) -> String {
        format!(
            "User #{} - @{}\nEmail: {}\nLast IP: {}",
            self.id, self.username, self.email, self.last_login_ip
        )
    }
}

impl<S> User<S>
where
    S: user_state::State<Username = Present>,
{
    fn get_username(&self) -> &str {
        &self.username
    }
}

impl<S> User<S>
where
    S: user_state::State<Email = Present>,
{
    fn get_email(&self) -> &str {
        &self.email
    }
}

fn main() {
    println!("=== Field Projection Example ===\n");

    println!("1. Creating a full user (with sensitive data):");
    let full_user: FullUser = User::new(1)
        .username("alice".into())
        .email("alice@example.com".into())
        .password_hash("$2b$12$abcdef...".into())
        .full_name("Alice Tzfati".into())
        .bio("Software engineer and Rust enthusiast".into())
        .avatar_url("https://example.com/alice.jpg".into())
        .last_login_ip("192.168.1.100".into())
        .api_key("sk_live_abc123xyz...".into());

    println!("   Full user created with {} fields", 9);
    println!("   Username: {}", full_user.username);
    println!("   Has sensitive data: password_hash, api_key, last_login_ip");

    println!("\n2. Public profile (safe for anyone to see):");
    let public = full_user.to_public_profile();
    println!("{}", public.display());
    println!("   ✓ No sensitive data exposed");
    println!("   ✓ Username: {}", public.get_username());

    println!("\n3. Admin view (for administrators):");
    let admin = full_user.to_admin_view();
    println!("{}", admin.display());
    println!("   ✓ Includes email and IP for admin purposes");
    println!("   ✓ Still hides password_hash and api_key");

    println!("\n4. Basic info (minimal data):");
    let basic = full_user.to_basic_info();
    println!("   User #{}: @{}", basic.id, basic.username);
    println!("   Email: {}", basic.get_email());
    println!("   ✓ Only essential identification fields");

    println!("\n5. JSON-safe serialization (custom fields only):");
    let json = full_user.to_json_safe();
    println!("   {}", json);
    println!("   ✓ Manually control which fields appear in output");

    println!("\n6. Type safety - can't access non-projected fields:");
    println!("   Public profile has username: {}", public.username);
    println!("   Admin view has email: {}", admin.email);

    println!("\n7. Creating partial users directly:");
    let minimal = User::new(2)
        .username("bob".into())
        .email("bob@example.com".into());
    println!("   Created user with only username and email");
    println!("   Username: {}", minimal.get_username());
    println!("   Email: {}", minimal.get_email());

    println!("\n8. Different views for different contexts:");

    let users = vec![
        User::new(1).username("alice".into()).email("alice@example.com".into()).full_name("Alice".into()).bio("Rust dev".into()),
        User::new(2).username("bob".into()).email("bob@example.com".into()).full_name("Bob".into()).bio("Go dev".into()),
        User::new(3).username("carol".into()).email("carol@example.com".into()).full_name("Carol".into()).bio("Python dev".into()),
    ];

    println!("\n   Public directory:");
    for user in &users {
        let public = User::new(user.id)
            .username(user.username.clone())
            .full_name(user.full_name.clone())
            .bio(user.bio.clone());
        println!("   - {}: {}", public.username, public.full_name);
    }

    println!("\n=== Key Benefits ===");
    println!("✓ Type-safe field projection");
    println!("✓ Sensitive data can be excluded from projections");
    println!("✓ Different views for different audiences");
    println!("✓ Compiler prevents access to non-projected fields");
    println!("✓ Explicit about what data is included");
}
