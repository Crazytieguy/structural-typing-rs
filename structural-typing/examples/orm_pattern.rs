//! # ORM Pattern Example
//!
//! This example demonstrates using structural typing with database operations.
//! A single struct represents a table, but with different fields present at different
//! stages of the data lifecycle.

use structural_typing::{structural, Present, Access};
use std::collections::HashMap;

#[structural]
#[derive(Debug, Clone)]
struct User {
    // ID and timestamps are only present after database insertion
    id: i64,
    created_at: String,
    updated_at: String,

    // These fields are always required
    #[always]
    username: String,
    #[always]
    email: String,

    // Optional profile fields
    full_name: String,
    bio: String,
    avatar_url: String,
}

// A user from the database has all fields populated (even if empty)
type DbUser = User<
    user_state::SetAvatarUrl<
        user_state::SetBio<
            user_state::SetFullName<
                user_state::SetUpdatedAt<
                    user_state::SetCreatedAt<
                        user_state::SetId<user_state::Empty>
                    >
                >
            >
        >
    >
>;

// Mock database - in real code, this would be a real database connection
struct Database {
    users: HashMap<i64, DbUser>,
    next_id: i64,
}

impl Database {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    /// Insert a new user - accepts any User regardless of which optional fields are set
    fn insert<S>(&mut self, user: User<S>) -> DbUser
    where
        S: user_state::State,
    {
        let id = self.next_id;
        self.next_id += 1;

        let now = "2024-01-01T00:00:00Z".to_string();

        // Create DbUser with database-managed fields and optional profile fields
        let db_user = User::new(user.username.clone(), user.email.clone())
            .id(id)
            .created_at(now.clone())
            .updated_at(now)
            .full_name(user.full_name.get().cloned().unwrap_or_default())
            .bio(user.bio.get().cloned().unwrap_or_default())
            .avatar_url(user.avatar_url.get().cloned().unwrap_or_default());

        self.users.insert(id, db_user.clone());
        db_user
    }

    /// Update a user - accepts partial updates and merges with existing data
    fn update<S>(&mut self, id: i64, updates: User<S>) -> Option<DbUser>
    where
        S: user_state::State,
    {
        let mut existing = self.users.get(&id)?.clone();

        // Use the update method to merge changes
        existing.update(updates);
        existing.updated_at = "2024-01-01T00:00:01Z".to_string();

        self.users.insert(id, existing.clone());
        Some(existing)
    }

    fn find(&self, id: i64) -> Option<&DbUser> {
        self.users.get(&id)
    }
}

// Methods that work on users with IDs (i.e., from the database)
impl<S> User<S>
where
    S: user_state::State<Id = Present>,
{
    fn display_with_id(&self) -> String {
        format!("User #{}: {}", self.id, self.username)
    }
}

// Methods that work on users with full profiles
impl<S> User<S>
where
    S: user_state::State<FullName = Present, Bio = Present>,
{
    fn display_profile(&self) -> String {
        format!(
            "{} (@{})\n{}",
            self.full_name, self.username, self.bio
        )
    }
}

fn main() {
    println!("=== ORM Pattern Example ===\n");

    let mut db = Database::new();

    // 1. Create new users with minimal data
    println!("1. Creating new users (no ID yet):");
    let new_user = User::new("alice".into(), "alice@example.com".into());
    println!("   New user: {} <{}>", new_user.username, new_user.email);
    println!("   Has ID? {}", <_ as Access<i64>>::get(&new_user.id).is_some());

    // 2. Insert into database - gets ID assigned
    println!("\n2. Inserting into database:");
    let db_user = db.insert(new_user);
    println!("   {}", db_user.display_with_id());
    println!("   Created at: {}", db_user.created_at);

    // 3. Create user with partial profile
    println!("\n3. Creating user with partial profile:");
    let new_user_with_profile = User::new("bob".into(), "bob@example.com".into())
        .full_name("Bob Smith".into());
    let bob = db.insert(new_user_with_profile);
    println!("   {}", bob.display_with_id());
    println!("   Full name: {}", bob.full_name);

    // 4. Update user with more profile information
    println!("\n4. Updating user with more info:");
    let updates = User::new("alice".into(), "alice@example.com".into())
        .full_name("Alice Tzfati".into())
        .bio("Rust enthusiast and open source contributor".into())
        .avatar_url("https://example.com/alice.jpg".into());

    if let Some(updated) = db.update(db_user.id, updates) {
        println!("   Updated: {}", updated.display_with_id());
        println!("   {}", updated.display_profile());
    }

    // 5. Partial updates - only bio
    println!("\n5. Partial update (bio only):");
    let bio_update = User::new("bob".into(), "bob@example.com".into())
        .bio("Software engineer".into());

    if let Some(updated) = db.update(bob.id, bio_update) {
        println!("   Updated bio for {}", updated.full_name);
        println!("   New bio: {}", updated.bio);
    }

    // 6. Query and display
    println!("\n6. Querying database:");
    for id in 1..=2 {
        if let Some(user) = db.find(id) {
            println!("   {}", user.display_with_id());
            if !user.bio.is_empty() {
                println!("      Bio: {}", user.bio);
            }
        }
    }

    println!("\n=== Key Benefits ===");
    println!("✓ Single struct for all stages of data lifecycle");
    println!("✓ Type-safe: can't use ID before insertion");
    println!("✓ Partial updates type-checked at compile time");
    println!("✓ Methods can require specific fields to be present");
}
