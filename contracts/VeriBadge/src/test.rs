#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, Symbol};

    #[test]
    fn test_happy_path_mint() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VeriBadgeContract);

        let issuer = Address::generate(&env);
        let recipient = Address::generate(&env);

        env.as_contract(&contract_id, || {
            VeriBadgeContract::init(env.clone());

            let id = VeriBadgeContract::mint_badge(
                env.clone(),
                issuer.clone(),
                recipient.clone(),
                Symbol::short("DEAN"),
                123
            );

            assert_eq!(id, 1);
        });
    }

    #[test]
    fn test_unauthorized_mint() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VeriBadgeContract);

        let issuer = Address::generate(&env);
        let recipient = Address::generate(&env);

        env.as_contract(&contract_id, || {
            VeriBadgeContract::init(env.clone());

            let result = std::panic::catch_unwind(|| {
                VeriBadgeContract::mint_badge(
                    env.clone(),
                    issuer.clone(),
                    recipient.clone(),
                    Symbol::short("FAIL"),
                    123
                );
            });

            assert!(result.is_err());
        });
    }

    #[test]
    fn test_state_storage() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VeriBadgeContract);

        let issuer = Address::generate(&env);
        let recipient = Address::generate(&env);

        env.as_contract(&contract_id, || {
            VeriBadgeContract::init(env.clone());

            let id = VeriBadgeContract::mint_badge(
                env.clone(),
                issuer.clone(),
                recipient.clone(),
                Symbol::short("CERT"),
                123
            );

            let badge = VeriBadgeContract::get_badge(env.clone(), id);
            assert_eq!(badge.recipient, recipient);
        });
    }

    #[test]
    fn test_verify_badge_true() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VeriBadgeContract);

        let issuer = Address::generate(&env);
        let recipient = Address::generate(&env);

        env.as_contract(&contract_id, || {
            VeriBadgeContract::init(env.clone());

            let id = VeriBadgeContract::mint_badge(
                env.clone(),
                issuer.clone(),
                recipient.clone(),
                Symbol::short("OK"),
                123
            );

            assert!(VeriBadgeContract::verify_badge(env.clone(), id, recipient));
        });
    }

    #[test]
    fn test_verify_badge_false() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VeriBadgeContract);

        let issuer = Address::generate(&env);
        let recipient = Address::generate(&env);
        let fake_user = Address::generate(&env);

        env.as_contract(&contract_id, || {
            VeriBadgeContract::init(env.clone());

            let id = VeriBadgeContract::mint_badge(
                env.clone(),
                issuer.clone(),
                recipient.clone(),
                Symbol::short("OK"),
                123
            );

            assert!(!VeriBadgeContract::verify_badge(env.clone(), id, fake_user));
        });
    }
}