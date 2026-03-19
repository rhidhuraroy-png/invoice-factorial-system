#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Map, symbol_short};

#[contract]
pub struct InvoiceFactoring;

#[contractimpl]
impl InvoiceFactoring {

    // Create an invoice
    pub fn create_invoice(
        env: Env,
        id: u64,
        seller: Address,
        amount: i128,
    ) {
        let key = symbol_short!("INV");

        let mut invoices: Map<u64, (Address, i128, bool)> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        invoices.set(id, (seller, amount, false));

        env.storage().instance().set(&key, &invoices);
    }

    // Factor an invoice (mark as financed)
    pub fn factor_invoice(
        env: Env,
        id: u64,
    ) {
        let key = symbol_short!("INV");

        let mut invoices: Map<u64, (Address, i128, bool)> =
            env.storage().instance().get(&key).unwrap();

        let (seller, amount, _status) = invoices.get(id).unwrap();

        invoices.set(id, (seller, amount, true));

        env.storage().instance().set(&key, &invoices);
    }

    // Get invoice details
    pub fn get_invoice(
        env: Env,
        id: u64,
    ) -> (Address, i128, bool) {
        let key = symbol_short!("INV");

        let invoices: Map<u64, (Address, i128, bool)> =
            env.storage().instance().get(&key).unwrap();

        invoices.get(id).unwrap()
    }
}