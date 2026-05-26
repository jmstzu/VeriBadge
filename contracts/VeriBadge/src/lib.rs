#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Badge {
    pub recipient: Address,
    pub title: Symbol,
    pub issuer: Address,
    pub timestamp: u64,
}

#[contracttype]
pub enum StorageKey {
    Badge(u64),
    Count,
}

#[contract]
pub struct VeriBadgeContract;

#[contractimpl]
impl VeriBadgeContract {

    // Initialize contract state
    pub fn init(env: Env) {
        env.storage().instance().set(&StorageKey::Count, &0u64);
    }

    // Mint achievement badge
    pub fn mint_badge(
        env: Env,
        issuer: Address,
        recipient: Address,
        title: Symbol,
        timestamp: u64,
    ) -> u64 {

        issuer.require_auth();

        let mut count: u64 = env
            .storage()
            .instance()
            .get(&StorageKey::Count)
            .unwrap_or(0);

        count += 1;

        let badge = Badge {
            recipient,
            title,
            issuer,
            timestamp,
        };

        env.storage()
            .instance()
            .set(&StorageKey::Badge(count), &badge);

        env.storage()
            .instance()
            .set(&StorageKey::Count, &count);

        count
    }

    // Safe verification (NO PANIC VERSION)
    pub fn verify_badge(env: Env, badge_id: u64, user: Address) -> bool {
        let badge: Option<Badge> = env
            .storage()
            .instance()
            .get(&StorageKey::Badge(badge_id));

        match badge {
            Some(b) => b.recipient == user,
            None => false,
        }
    }

    // Safe getter
    pub fn get_badge(env: Env, badge_id: u64) -> Option<Badge> {
        env.storage()
            .instance()
            .get(&StorageKey::Badge(badge_id))
    }
}