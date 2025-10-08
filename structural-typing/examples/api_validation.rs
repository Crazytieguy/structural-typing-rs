//! # API Validation Example
//!
//! This example demonstrates using structural typing for multi-stage validation.
//! User input progresses through validation stages, with the type system ensuring
//! that only validated data can be used in sensitive operations.

use structural_typing::{structural, Present, Access};
use std::collections::HashSet;

#[structural]
#[derive(Debug, Clone)]
struct UserRegistration {
    #[always]
    request_id: String,

    username: String,
    email: String,
    password: String,
    age: u8,
    terms_accepted: bool,
}

type WithBasicValidation = UserRegistration<
    userregistration_state::SetUsername<
        userregistration_state::SetEmail<
            userregistration_state::Empty
        >
    >
>;

type WithSecurityValidation = UserRegistration<
    userregistration_state::SetPassword<
        userregistration_state::SetUsername<
            userregistration_state::SetEmail<
                userregistration_state::Empty
            >
        >
    >
>;

type FullyValidated = UserRegistration<
    userregistration_state::SetTermsAccepted<
        userregistration_state::SetAge<
            userregistration_state::SetPassword<
                userregistration_state::SetUsername<
                    userregistration_state::SetEmail<
                        userregistration_state::Empty
                    >
                >
            >
        >
    >
>;

#[derive(Debug)]
#[allow(dead_code)]
enum ValidationError {
    InvalidEmail(String),
    InvalidUsername(String),
    WeakPassword(String),
    AgeTooYoung,
    TermsNotAccepted,
}

struct ValidationService {
    used_usernames: HashSet<String>,
    used_emails: HashSet<String>,
}

impl ValidationService {
    fn new() -> Self {
        Self {
            used_usernames: HashSet::new(),
            used_emails: HashSet::new(),
        }
    }

    fn validate_email(&self, email: &str) -> Result<String, ValidationError> {
        if !email.contains('@') || !email.contains('.') {
            return Err(ValidationError::InvalidEmail("Email must contain @ and .".into()));
        }

        if self.used_emails.contains(email) {
            return Err(ValidationError::InvalidEmail("Email already registered".into()));
        }

        Ok(email.to_lowercase())
    }

    fn validate_username(&self, username: &str) -> Result<String, ValidationError> {
        if username.len() < 3 {
            return Err(ValidationError::InvalidUsername("Username must be at least 3 characters".into()));
        }

        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(ValidationError::InvalidUsername("Username can only contain letters, numbers, and underscores".into()));
        }

        if self.used_usernames.contains(username) {
            return Err(ValidationError::InvalidUsername("Username already taken".into()));
        }

        Ok(username.to_string())
    }

    fn validate_password(&self, password: &str) -> Result<String, ValidationError> {
        if password.len() < 8 {
            return Err(ValidationError::WeakPassword("Password must be at least 8 characters".into()));
        }

        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(ValidationError::WeakPassword("Password must contain uppercase letter".into()));
        }

        if !password.chars().any(|c| c.is_numeric()) {
            return Err(ValidationError::WeakPassword("Password must contain a number".into()));
        }

        Ok(password.to_string())
    }

    fn validate_age(&self, age: u8) -> Result<u8, ValidationError> {
        if age < 13 {
            return Err(ValidationError::AgeTooYoung);
        }
        Ok(age)
    }

    fn validate_terms(&self, accepted: bool) -> Result<bool, ValidationError> {
        if !accepted {
            return Err(ValidationError::TermsNotAccepted);
        }
        Ok(accepted)
    }

    fn stage1_basic_info<S>(
        &self,
        raw: UserRegistration<S>,
    ) -> Result<WithBasicValidation, ValidationError>
    where
        S: userregistration_state::State,
    {
        let email = self.validate_email(
            raw.email.get().ok_or_else(|| ValidationError::InvalidEmail("Email required".into()))?
        )?;
        let username = self.validate_username(
            raw.username.get().ok_or_else(|| ValidationError::InvalidUsername("Username required".into()))?
        )?;

        Ok(UserRegistration::new(raw.request_id.clone())
            .email(email)
            .username(username))
    }

    fn stage2_security<S>(
        &self,
        raw: UserRegistration<S>,
        validated_basic: WithBasicValidation,
    ) -> Result<WithSecurityValidation, ValidationError>
    where
        S: userregistration_state::State,
    {
        let password = self.validate_password(
            raw.password.get().ok_or_else(|| ValidationError::WeakPassword("Password required".into()))?
        )?;

        Ok(UserRegistration::new(validated_basic.request_id.clone())
            .email(validated_basic.email.clone())
            .username(validated_basic.username.clone())
            .password(password))
    }

    fn stage3_finalize<S>(
        &self,
        raw: UserRegistration<S>,
        validated_security: WithSecurityValidation,
    ) -> Result<FullyValidated, ValidationError>
    where
        S: userregistration_state::State,
    {
        let age = self.validate_age(
            *raw.age.get().ok_or(ValidationError::AgeTooYoung)?
        )?;
        let terms = self.validate_terms(
            *raw.terms_accepted.get().ok_or(ValidationError::TermsNotAccepted)?
        )?;

        Ok(UserRegistration::new(validated_security.request_id.clone())
            .email(validated_security.email.clone())
            .username(validated_security.username.clone())
            .password(validated_security.password.clone())
            .age(age)
            .terms_accepted(terms))
    }

    fn register_user(&mut self, validated: FullyValidated) {
        self.used_emails.insert(validated.email.clone());
        self.used_usernames.insert(validated.username.clone());
        println!("✓ User registered: {} ({})", validated.username, validated.email);
    }
}

impl<S> UserRegistration<S>
where
    S: userregistration_state::State<
        Email = Present,
        Username = Present,
        Password = Present,
    >,
{
    fn can_login(&self) -> bool {
        !self.email.is_empty() && !self.username.is_empty() && !self.password.is_empty()
    }
}

fn main() {
    println!("=== API Validation Example ===\n");

    let mut service = ValidationService::new();

    println!("1. Valid registration - all stages pass:");
    let raw = UserRegistration::new("req-001".into())
        .username("alice_2024".into())
        .email("alice@example.com".into())
        .password("SecurePass123".into())
        .age(25)
        .terms_accepted(true);

    match service.stage1_basic_info(raw.clone())
        .and_then(|v1| service.stage2_security(raw.clone(), v1))
        .and_then(|v2| service.stage3_finalize(raw.clone(), v2))
    {
        Ok(validated) => {
            println!("   All validations passed!");
            println!("   Can login: {}", validated.can_login());
            service.register_user(validated);
        }
        Err(e) => println!("   Validation failed: {:?}", e),
    }

    println!("\n2. Invalid email:");
    let raw = UserRegistration::new("req-002".into())
        .username("bob".into())
        .email("not-an-email".into())
        .password("SecurePass123".into())
        .age(30)
        .terms_accepted(true);

    match service.stage1_basic_info(raw) {
        Ok(_) => println!("   Stage 1 passed (unexpected)"),
        Err(e) => println!("   ✗ Stage 1 failed: {:?}", e),
    }

    println!("\n3. Weak password:");
    let raw = UserRegistration::new("req-003".into())
        .username("charlie".into())
        .email("charlie@example.com".into())
        .password("weak".into())
        .age(28)
        .terms_accepted(true);

    match service.stage1_basic_info(raw.clone())
        .and_then(|v1| service.stage2_security(raw.clone(), v1))
    {
        Ok(_) => println!("   Stage 2 passed (unexpected)"),
        Err(e) => println!("   ✗ Stage 2 failed: {:?}", e),
    }

    println!("\n4. Partial validation - stage by stage:");
    let raw = UserRegistration::new("req-004".into())
        .username("diana".into())
        .email("diana@example.com".into())
        .password("GoodPass789".into())
        .age(35)
        .terms_accepted(true);

    println!("   Starting validation pipeline...");

    let after_stage1 = service.stage1_basic_info(raw.clone()).unwrap();
    println!("   ✓ Stage 1 (Basic Info): Email and username validated");

    let after_stage2 = service.stage2_security(raw.clone(), after_stage1).unwrap();
    println!("   ✓ Stage 2 (Security): Password validated");
    println!("   Can login: {}", after_stage2.can_login());

    let after_stage3 = service.stage3_finalize(raw.clone(), after_stage2).unwrap();
    println!("   ✓ Stage 3 (Finalize): Age and terms validated");

    service.register_user(after_stage3);

    println!("\n5. Duplicate username:");
    let raw = UserRegistration::new("req-005".into())
        .username("alice_2024".into())
        .email("alice2@example.com".into())
        .password("AnotherPass456".into())
        .age(22)
        .terms_accepted(true);

    match service.stage1_basic_info(raw) {
        Ok(_) => println!("   Stage 1 passed (unexpected)"),
        Err(e) => println!("   ✗ Stage 1 failed: {:?}", e),
    }

    println!("\n6. Merging partial data from different sources:");
    let basic_info = UserRegistration::new("req-006".into())
        .username("eve".into())
        .email("eve@example.com".into());

    let security_info = UserRegistration::new("req-006".into())
        .password("StrongPass999".into());

    let profile_info = UserRegistration::new("req-006".into())
        .age(19)
        .terms_accepted(true);

    let merged = basic_info
        .merge(security_info)
        .merge(profile_info);

    match service.stage1_basic_info(merged.clone())
        .and_then(|v1| service.stage2_security(merged.clone(), v1))
        .and_then(|v2| service.stage3_finalize(merged.clone(), v2))
    {
        Ok(validated) => {
            println!("   ✓ Merged data validated successfully!");
            service.register_user(validated);
        }
        Err(e) => println!("   ✗ Validation failed: {:?}", e),
    }

    println!("\n=== Key Benefits ===");
    println!("✓ Type-safe validation pipeline");
    println!("✓ Each stage requires specific fields");
    println!("✓ Can't use unvalidated data in sensitive operations");
    println!("✓ Validation stages can be composed or run separately");
    println!("✓ Data from different sources can be merged");
}
