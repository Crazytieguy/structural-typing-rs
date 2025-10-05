use structural_typing::{structural, Access, Present};

#[structural]
struct UserProfile {
    username: String,
    email: String,
    age: u8,
    bio: String,
    verified: bool,
}

impl<S> UserProfile<S>
where
    S: userprofile_state::State<Username = Present>,
{
    fn display_name(&self) -> &str {
        &self.username
    }

    fn is_complete(&self) -> bool {
        self.email.get().is_some()
            && self.age.get().is_some()
            && self.bio.get().is_some()
    }
}

impl<S> UserProfile<S>
where
    S: userprofile_state::State<Username = Present, Email = Present>,
{
    fn contact_info(&self) -> String {
        format!("{} <{}>", self.username, self.email)
    }
}

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

    println!("1. Building a profile step by step:");
    let profile = UserProfile::empty();
    println!("   Empty profile created");

    let profile = profile.username("alice".into());
    println!("   Added username: {}", profile.display_name());
    println!("   Profile complete? {}", profile.is_complete());

    let profile = profile.email("alice@example.com".into());
    println!("   Added email");
    println!("   Contact info: {}", profile.contact_info());
    println!("   Profile complete? {}", profile.is_complete());

    let profile = profile.age(28).bio("Rust enthusiast".into()).verified(true);
    println!("   Added remaining fields");
    println!("   Profile complete? {}", profile.is_complete());
    println!("\n   Full profile:\n{}\n", profile.full_profile());

    println!("2. Merging partial profiles:");
    let basic_info = UserProfile::empty()
        .username("bob".into())
        .email("bob@example.com".into());

    let additional_info = UserProfile::empty()
        .age(35)
        .bio("Software engineer".into())
        .verified(false);

    let merged = basic_info.merge(additional_info);
    println!("   Merged profile: {}\n", merged.contact_info());

    println!("3. Runtime field checking with require_*:");
    let partial_no_email = UserProfile::empty().username("charlie".into());

    match partial_no_email.require_email() {
        Some(_) => println!("   Email is present"),
        None => println!("   Email is missing - cannot create complete profile"),
    }

    let partial_with_email = UserProfile::empty()
        .username("charlie".into())
        .email("charlie@example.com".into());

    match partial_with_email.require_email() {
        Some(p) => println!("   Email found: {}", p.contact_info()),
        None => println!("   This won't happen"),
    }

    println!("\n4. Type safety demonstration:");
    let incomplete = UserProfile::empty().username("dave".into());
    println!("   Incomplete profile can use display_name(): {}", incomplete.display_name());
    println!("   But NOT contact_info() - won't compile without email!");
    println!("   And definitely NOT full_profile() - needs all fields!");

    println!("\n=== All features demonstrated! ===");
}
