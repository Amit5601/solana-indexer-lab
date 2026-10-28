use std::collections::HashMap;

struct TransactionStore {
    transactions: HashMap<String, Transaction>,
}

#[derive(Debug, PartialEq)]
struct Transaction {
    signature: String,
    slot: u64,
}
#[derive(Debug, PartialEq)]
enum SaveOutcome {
    Inserted,
    AlreadyExists,
}

impl TransactionStore {
    fn save(&mut self, transaction: Transaction) -> SaveOutcome {
        if self.transactions.contains_key(&transaction.signature) {
            return SaveOutcome::AlreadyExists;
        }

        self.transactions
            .insert(transaction.signature.clone(), transaction);
        SaveOutcome::Inserted
    }

    fn get(&self, signature: &str) -> Option<&Transaction> {
        self.transactions.get(signature)
    }

}

impl Transaction {
    fn get_slot (&self) -> u64 {
        self.slot
    }
}

fn main() {
    let mut transaction_store = TransactionStore {
        transactions: HashMap::new(),
    };

    let transaction = Transaction {
        signature: String::from("abc123"),
        slot: 101,
    };

    let transaction_1 = Transaction {
        signature: String::from("abc123"),
        slot: 102,
    };

    let output = transaction_store.save(transaction);
    let output_1 = transaction_store.save(transaction_1);

    match output {
        SaveOutcome::Inserted => println!("Transaction saved"),
        SaveOutcome::AlreadyExists => println!("Duplicate transaction skipped"),
    }
    match output_1 {
        SaveOutcome::Inserted => println!("Transaction saved"),
        SaveOutcome::AlreadyExists => println!("Duplicate transaction skipped"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_new_transaction() {
        let mut transaction_store = TransactionStore {
            transactions: HashMap::new(),
        };

        let transaction = Transaction {
            signature: String::from("abc123"),
            slot: 101,
        };

        let outcome = transaction_store.save(transaction);

        assert_eq!(outcome, SaveOutcome::Inserted);
    }
    #[test]
    fn reject_duplicate_transactions() {
        let mut transaction_store = TransactionStore {
            transactions: HashMap::new(),
        };

        let transaction = Transaction {
            signature: String::from("abc123"),
            slot: 101,
        };

        let transaction_1 = Transaction {
            signature: String::from("abc123"),
            slot: 102,
        };

        transaction_store.save(transaction);
        let outcome_1 = transaction_store.save(transaction_1);

        assert_eq!(outcome_1, SaveOutcome::AlreadyExists);

        let value = transaction_store
            .get("abc123")
            .expect("original transaction should still exist");

        assert_eq!(value.slot, 101);
    }

    #[test]
    fn returns_none_for_unknown_signature() {
        let transaction_store = TransactionStore {
            transactions: HashMap::new(),
        };

        let result = transaction_store.get("abcd1234");

        assert_eq!(result, None);
    }

    #[test]
    fn return_correct_slot () {
           let mut transaction_store = TransactionStore {
               transactions: HashMap::new(), 
              };

        let transaction_1 = Transaction {
            signature: String::from("xyz123"),
            slot: 111
        };

        let result = transaction_store.save(transaction_1);

        let slot = transaction_store
            .get("xyz123")
            .expect("saved transaction should exist")
            .get_slot();

        assert_eq!(result, SaveOutcome::Inserted);
        assert_eq!(slot, 111);
    }
}