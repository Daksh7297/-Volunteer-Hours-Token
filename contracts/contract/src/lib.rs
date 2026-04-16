#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, symbol_short, log,
};

// ============================================================
// DATA STRUCTURES - Kya Kya Store Hoga
// ============================================================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Volunteer {
    pub address: Address,
    pub name: Symbol,
    pub total_hours: u64,
    pub total_tokens: i128,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct HoursLog {
    pub volunteer: Address,
    pub hours: u64,
    pub description: Symbol,
    pub timestamp: u64,
    pub approved: bool,
    pub tokens_minted: bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Reward {
    pub id: u64,
    pub name: Symbol,
    pub cost: i128,
    pub available: u64,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Redemption {
    pub volunteer: Address,
    pub reward_id: u64,
    pub tokens_spent: i128,
    pub timestamp: u64,
}

// ============================================================
// STORAGE KEYS - Database Ki Keys
// ============================================================

#[contracttype]
pub enum DataKey {
    Admin,
    TokenName,
    TokenSymbol,
    TotalSupply,
    TokensPerHour,
    IsInitialized,
    Balance(Address),
    VolunteerInfo(Address),
    VolunteerCount,
    HoursLog(u64),
    HoursLogCount,
    RewardItem(u64),
    RewardCount,
    RedemptionLog(u64),
    RedemptionCount,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct VolunteerHoursToken;

#[contractimpl]
impl VolunteerHoursToken {

    // ========================================================
    // INITIALIZE - Token Setup (Sirf 1 Baar Call Hoga)
    // ========================================================

    pub fn initialize(
        env: Env,
        admin: Address,
        token_name: Symbol,
        token_symbol: Symbol,
        tokens_per_hour: i128,
    ) {
        if env.storage().instance().has(&DataKey::IsInitialized) {
            panic!("Already initialized");
        }

        admin.require_auth();

        if tokens_per_hour <= 0 {
            panic!("Tokens per hour must be positive");
        }

        env.storage().instance().set(&DataKey::IsInitialized, &true);
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenName, &token_name);
        env.storage().instance().set(&DataKey::TokenSymbol, &token_symbol);
        env.storage().instance().set(&DataKey::TokensPerHour, &tokens_per_hour);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
        env.storage().instance().set(&DataKey::VolunteerCount, &0u64);
        env.storage().instance().set(&DataKey::HoursLogCount, &0u64);
        env.storage().instance().set(&DataKey::RewardCount, &0u64);
        env.storage().instance().set(&DataKey::RedemptionCount, &0u64);

        log!(&env, "VHT Token initialized successfully");
    }

    // ========================================================
    // REGISTER VOLUNTEER - Naya Volunteer Register Kare
    // ========================================================

    pub fn register_volunteer(
        env: Env,
        volunteer_address: Address,
        name: Symbol,
    ) {
        volunteer_address.require_auth();

        if env.storage().instance().has(
            &DataKey::VolunteerInfo(volunteer_address.clone())
        ) {
            panic!("Volunteer already registered");
        }

        let volunteer = Volunteer {
            address: volunteer_address.clone(),
            name: name.clone(),
            total_hours: 0,
            total_tokens: 0,
            is_active: true,
        };

        env.storage().instance().set(
            &DataKey::VolunteerInfo(volunteer_address.clone()),
            &volunteer,
        );

        env.storage().instance().set(
            &DataKey::Balance(volunteer_address.clone()),
            &0i128,
        );

        let count: u64 = env.storage().instance()
            .get(&DataKey::VolunteerCount)
            .unwrap_or(0);
        env.storage().instance().set(
            &DataKey::VolunteerCount,
            &(count + 1),
        );

        log!(&env, "Volunteer registered: {}", name);
    }

    // ========================================================
    // LOG HOURS - Volunteer Apne Kaam Ke Hours Log Kare
    // ========================================================

    pub fn log_hours(
        env: Env,
        volunteer: Address,
        hours: u64,
        description: Symbol,
    ) -> u64 {
        volunteer.require_auth();

        if !env.storage().instance().has(
            &DataKey::VolunteerInfo(volunteer.clone())
        ) {
            panic!("Volunteer not registered");
        }

        if hours == 0 || hours > 24 {
            panic!("Hours must be between 1 and 24");
        }

        let log_id: u64 = env.storage().instance()
            .get(&DataKey::HoursLogCount)
            .unwrap_or(0);

        let hours_log = HoursLog {
            volunteer: volunteer.clone(),
            hours,
            description,
            timestamp: env.ledger().timestamp(),
            approved: false,
            tokens_minted: false,
        };

        env.storage().instance().set(
            &DataKey::HoursLog(log_id),
            &hours_log,
        );
        env.storage().instance().set(
            &DataKey::HoursLogCount,
            &(log_id + 1),
        );

        log!(&env, "Hours logged: {} hours, Log ID: {}", hours, log_id);

        log_id
    }

    // ========================================================
    // APPROVE HOURS - Admin Hours Approve Kare + Auto Mint
    // ========================================================

    pub fn approve_hours(env: Env, log_id: u64) {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Admin not set");
        admin.require_auth();

        let mut hours_log: HoursLog = env.storage().instance()
            .get(&DataKey::HoursLog(log_id))
            .expect("Hours log not found");

        if hours_log.approved {
            panic!("Hours already approved");
        }

        hours_log.approved = true;
        hours_log.tokens_minted = true;

        let rate: i128 = env.storage().instance()
            .get(&DataKey::TokensPerHour)
            .unwrap_or(1);
        let tokens_earned: i128 = (hours_log.hours as i128) * rate;

        let current_balance: i128 = env.storage().instance()
            .get(&DataKey::Balance(hours_log.volunteer.clone()))
            .unwrap_or(0);
        env.storage().instance().set(
            &DataKey::Balance(hours_log.volunteer.clone()),
            &(current_balance + tokens_earned),
        );

        let mut vol_info: Volunteer = env.storage().instance()
            .get(&DataKey::VolunteerInfo(hours_log.volunteer.clone()))
            .expect("Volunteer not found");
        vol_info.total_hours += hours_log.hours;
        vol_info.total_tokens += tokens_earned;
        env.storage().instance().set(
            &DataKey::VolunteerInfo(hours_log.volunteer.clone()),
            &vol_info,
        );

        let supply: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage().instance().set(
            &DataKey::TotalSupply,
            &(supply + tokens_earned),
        );

        env.storage().instance().set(
            &DataKey::HoursLog(log_id),
            &hours_log,
        );

        log!(
            &env,
            "Approved! {} VHT minted for log #{}",
            tokens_earned,
            log_id
        );
    }

    // ========================================================
    // ADD REWARD - Admin Naya Reward Add Kare
    // ========================================================

    pub fn add_reward(
        env: Env,
        name: Symbol,
        cost: i128,
        available: u64,
    ) -> u64 {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Admin not set");
        admin.require_auth();

        if cost <= 0 {
            panic!("Cost must be positive");
        }
        if available == 0 {
            panic!("Available must be at least 1");
        }

        let reward_id: u64 = env.storage().instance()
            .get(&DataKey::RewardCount)
            .unwrap_or(0);

        let reward = Reward {
            id: reward_id,
            name: name.clone(),
            cost,
            available,
            is_active: true,
        };

        env.storage().instance().set(
            &DataKey::RewardItem(reward_id),
            &reward,
        );
        env.storage().instance().set(
            &DataKey::RewardCount,
            &(reward_id + 1),
        );

        log!(&env, "Reward added: {} (Cost: {} VHT)", name, cost);

        reward_id
    }

    // ========================================================
    // REDEEM REWARD - Volunteer Tokens Se Reward Le
    // ========================================================

    pub fn redeem_reward(
        env: Env,
        volunteer: Address,
        reward_id: u64,
    ) -> u64 {
        volunteer.require_auth();

        let mut reward: Reward = env.storage().instance()
            .get(&DataKey::RewardItem(reward_id))
            .expect("Reward not found");

        if !reward.is_active {
            panic!("Reward is not active");
        }
        if reward.available == 0 {
            panic!("Reward is out of stock");
        }

        let balance: i128 = env.storage().instance()
            .get(&DataKey::Balance(volunteer.clone()))
            .unwrap_or(0);

        if balance < reward.cost {
            panic!("Insufficient token balance");
        }

        env.storage().instance().set(
            &DataKey::Balance(volunteer.clone()),
            &(balance - reward.cost),
        );

        reward.available -= 1;
        if reward.available == 0 {
            reward.is_active = false;
        }
        env.storage().instance().set(
            &DataKey::RewardItem(reward_id),
            &reward,
        );

        let supply: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage().instance().set(
            &DataKey::TotalSupply,
            &(supply - reward.cost),
        );

        let redemption_id: u64 = env.storage().instance()
            .get(&DataKey::RedemptionCount)
            .unwrap_or(0);

        let redemption = Redemption {
            volunteer: volunteer.clone(),
            reward_id,
            tokens_spent: reward.cost,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(
            &DataKey::RedemptionLog(redemption_id),
            &redemption,
        );
        env.storage().instance().set(
            &DataKey::RedemptionCount,
            &(redemption_id + 1),
        );

        log!(
            &env,
            "Reward redeemed! {} VHT spent on reward #{}",
            reward.cost,
            reward_id
        );

        redemption_id
    }

    // ========================================================
    // TRANSFER - Volunteer Tokens Bheje Doosre Ko
    // ========================================================

    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }
        if from == to {
            panic!("Cannot transfer to yourself");
        }

        let from_balance: i128 = env.storage().instance()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env.storage().instance()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage().instance().set(
            &DataKey::Balance(from.clone()),
            &(from_balance - amount),
        );
        env.storage().instance().set(
            &DataKey::Balance(to.clone()),
            &(to_balance + amount),
        );

        log!(&env, "Transferred {} VHT tokens", amount);
    }

    // ========================================================
    // VIEW FUNCTIONS - Dekhne Ke Liye (Read Only)
    // ========================================================

    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().instance()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    pub fn get_volunteer(env: Env, address: Address) -> Volunteer {
        env.storage().instance()
            .get(&DataKey::VolunteerInfo(address))
            .expect("Volunteer not found")
    }

    pub fn get_reward(env: Env, reward_id: u64) -> Reward {
        env.storage().instance()
            .get(&DataKey::RewardItem(reward_id))
            .expect("Reward not found")
    }

    pub fn get_hours_log(env: Env, log_id: u64) -> HoursLog {
        env.storage().instance()
            .get(&DataKey::HoursLog(log_id))
            .expect("Hours log not found")
    }

    pub fn get_volunteer_count(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::VolunteerCount)
            .unwrap_or(0)
    }

    pub fn get_reward_count(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::RewardCount)
            .unwrap_or(0)
    }

    pub fn name(env: Env) -> Symbol {
        env.storage().instance()
            .get(&DataKey::TokenName)
            .unwrap_or(symbol_short!("VHT"))
    }

    pub fn symbol(env: Env) -> Symbol {
        env.storage().instance()
            .get(&DataKey::TokenSymbol)
            .unwrap_or(symbol_short!("VHT"))
    }

    pub fn get_rate(env: Env) -> i128 {
        env.storage().instance()
            .get(&DataKey::TokensPerHour)
            .unwrap_or(1)
    }

    pub fn get_redemption(env: Env, redemption_id: u64) -> Redemption {
        env.storage().instance()
            .get(&DataKey::RedemptionLog(redemption_id))
            .expect("Redemption not found")
    }

    pub fn get_redemption_count(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::RedemptionCount)
            .unwrap_or(0)
    }
}