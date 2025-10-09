//! # Comprehensive Example
//!
//! This example demonstrates the key features of structural typing:
//! - Type-state pattern with compile-time field presence tracking
//! - Builder-like API with method chaining
//! - Conditional methods that only work when specific fields are present
//! - Runtime field checking with `require_*` methods
//! - Merging partial structs

use structural_typing::{structural, Access, Present};

#[structural]
struct UserProfile {
    username: String,
    email: String,
    age: u8,
    bio: String,
    verified: bool,
}

// Methods available when ONLY username is present
impl<S> UserProfile<S>
where
    S: userprofile_state::State<Username = Present>,
{
    /// Get the display name - only requires username
    fn display_name(&self) -> &str {
        &self.username
    }

    /// Check if profile is complete - uses Access trait to check optional fields
    fn is_complete(&self) -> bool {
        self.email.get().is_some()
            && self.age.get().is_some()
            && self.bio.get().is_some()
    }
}

// Methods available when BOTH username AND email are present
impl<S> UserProfile<S>
where
    S: userprofile_state::State<Username = Present, Email = Present>,
{
    /// Get contact info - requires both username and email
    fn contact_info(&self) -> String {
        format!("{} <{}>", self.username, self.email)
    }
}

// Methods available when ALL fields are present
impl<S> UserProfile<S>
where
    S: userprofile_state::State<
        Username = Present,
        Email = Present,
        Age = Present,
        Bio = Present,
        Verified = Present,
    >,
{
    /// Get full profile - requires all fields
    #[allow(dead_code)]
    fn full_profile(&self) -> String {
        let status = if self.verified { "verified" } else { "unverified" };
        format!(
            "User: {} ({})\nEmail: {}\nAge: {}\nBio: {}\nStatus: {}",
            self.username, status, self.email, self.age, self.bio, status
        )
    }
}

fn main() {
    println!("=== Structural Typing Showcase ===\n");

    // 1. Builder pattern with type-state
    println!("1. Building a profile step by step:");

    // Start with an empty profile
    let profile = UserProfile::empty();

    // Add username - now we can call display_name()
    let profile = profile.username("alice".into());
    println!("   Username: {}", profile.display_name());

    // Add email - now we can call contact_info()
    let profile = profile.email("alice@example.com".into());
    println!("   Contact: {}", profile.contact_info());

    // Add remaining fields - now we can call full_profile()
    let profile = profile.age(28).bio("Rust enthusiast".into()).verified(true);
    println!("   Complete: {}\n", profile.is_complete());

    // 2. Merging partial structs
    println!("2. Merging partial profiles:");

    // Create two partial profiles from different sources
    let basic_info = UserProfile::empty()
        .username("bob".into())
        .email("bob@example.com".into());

    let additional_info = UserProfile::empty()
        .age(35)
        .bio("Software engineer".into())
        .verified(false);

    // Merge them together - later values take precedence
    let merged = basic_info.merge(additional_info);
    println!("   Merged: {}\n", merged.contact_info());

    // 3. Runtime field checking
    println!("3. Runtime field checking with require_*:");

    // Profile without email - require_email() returns None
    let partial_no_email = UserProfile::empty().username("charlie".into());
    match partial_no_email.require_email() {
        Some(_) => println!("   Has email"),
        None => println!("   Missing email"),
    }

    // Profile with email - require_email() returns Some with new type
    let partial_with_email = UserProfile::empty()
        .username("charlie".into())
        .email("charlie@example.com".into());

    match partial_with_email.require_email() {
        Some(p) => println!("   Has email: {}\n", p.contact_info()),
        None => unreachable!(),
    }

    // 4. Type safety demonstration
    println!("4. Type safety - methods require specific fields:");
    let incomplete = UserProfile::empty().username("dave".into());
    println!("   ✓ display_name() works: {}", incomplete.display_name());
    println!("   ✗ contact_info() won't compile - needs email!");
    println!("   ✗ full_profile() won't compile - needs all fields!");

    println!("\n=== All features demonstrated! ===");
}
