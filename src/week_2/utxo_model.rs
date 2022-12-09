// UTXO Model
struct Txo {
    owner: String,
    amount: u128,
    spent: bool,
}

impl Txo {
    fn new(owner: String, amount: u128) -> Self {
        Self {
            owner,
            amount,
            spent: false,
        }
    }

    fn spend(&mut self) {
        self.spent = true
    }
}

struct Transaction {
    input_utxos: Vec<Txo>,
    output_utxos: Vec<Txo>,
    fee: u128,
}

#[derive(PartialEq, Debug)]
enum TransactionExecutionError {
    UtxoAlreadySpent,
    InsufficientInputs,
}

impl Transaction {
    fn new(input_utxos: Vec<Txo>, output_utxos: Vec<Txo>) -> Self {
        Self {
            input_utxos,
            output_utxos,
            fee: 0,
        }
    }

    fn execute(&mut self) -> Option<TransactionExecutionError> {
        let spent = self
            .input_utxos
            .iter()
            .fold(false, |acc, curr| curr.spent || acc);

        if spent {
            return Some(TransactionExecutionError::UtxoAlreadySpent);
        }

        let input_total: u128 = self.input_utxos.iter().map(|curr| curr.amount).sum();
        let output_total: u128 = self.output_utxos.iter().map(|curr| curr.amount).sum();

        if input_total < output_total {
            return Some(TransactionExecutionError::InsufficientInputs);
        }

        self.input_utxos.iter_mut().for_each(|txo| txo.spend());

        self.fee = input_total - output_total;

        None
    }
}

mod tests {
    mod txo {
        use crate::week_2::utxo_model::Txo;

        const address: &str = "1DBS97W3jWw6FnAqdduK1NW6kFo3Aid1N6";
        const amount: u128 = 10;

        mod new {
            use crate::week_2::utxo_model::Txo;

            use super::{address, amount};

            #[test]
            fn should_set_the_owner() {
                // Act
                let txo = Txo::new(String::from(address), amount);

                // Assert
                assert_eq!(txo.owner, address)
            }

            #[test]
            fn should_set_the_amout() {
                // Act
                let txo = Txo::new(String::from(address), amount);

                // Assert
                assert_eq!(txo.amount, amount)
            }

            #[test]
            fn should_set_the_spent_to_false() {
                // Act
                let txo = Txo::new(String::from(address), amount);

                // Assert
                assert_eq!(txo.spent, false)
            }
        }

        mod spend {
            use crate::week_2::utxo_model::Txo;

            use super::{address, amount};

            #[test]
            fn should_set_the_spent_to_true() {
                // Arrange
                let mut txo = Txo::new(String::from(address), amount);

                // Act
                txo.spend();

                // Assert
                assert_eq!(txo.spent, true)
            }
        }
    }

    mod transaction {
        const from_address: &str = "1DBS97W3jWw6FnAqdduK1NW6kFo3Aid1N6";
        const to_address: &str = "12ruWjb4naCME5QhjrQSJuS5disgME22fe";

        mod with_unspent_input {
            use crate::week_2::utxo_model::{Transaction, TransactionExecutionError, Txo};

            use super::{from_address, to_address};

            #[test]
            fn should_execute() {
                // Arrange
                let mut transaction = Transaction::new(
                    vec![
                        Txo::new(String::from(from_address), 5),
                        Txo::new(String::from(from_address), 7),
                    ],
                    vec![Txo::new(String::from(to_address), 10)],
                );

                // Act
                let res = transaction.execute();

                // Assert
                assert!(res.is_none());
            }

            #[test]
            fn should_mark_inputs_as_spent() {
                // Arrange
                let mut transaction = Transaction::new(
                    vec![
                        Txo::new(String::from(from_address), 5),
                        Txo::new(String::from(from_address), 7),
                    ],
                    vec![Txo::new(String::from(to_address), 10)],
                );

                // Act
                let res = transaction.execute();

                // Assert
                transaction
                    .input_utxos
                    .iter()
                    .for_each(|txo| assert!(txo.spent));
                assert!(res.is_none());
            }

            #[test]
            fn should_update_the_fee_correctly() {
                // Arrange
                let mut transaction = Transaction::new(
                    vec![
                        Txo::new(String::from(from_address), 5),
                        Txo::new(String::from(from_address), 7),
                    ],
                    vec![Txo::new(String::from(to_address), 10)],
                );

                // Act
                let res = transaction.execute();

                // Assert
                assert!(res.is_none());
                assert_eq!(transaction.fee, 2);
            }
        }

        mod with_a_spent_input {
            use crate::week_2::utxo_model::{
                tests::transaction::{from_address, to_address},
                Transaction, TransactionExecutionError, Txo,
            };

            #[test]
            fn should_return_utxo_already_spent() {
                // Arrange
                let mut txo = Txo::new(String::from(from_address), 10);
                txo.spend();

                let mut transaction = Transaction::new(
                    vec![
                        Txo::new(String::from(from_address), 5),
                        Txo::new(String::from(from_address), 7),
                        txo,
                    ],
                    vec![Txo::new(String::from(to_address), 22)],
                );

                // Act
                let res = transaction.execute();

                // Assert
                assert!(res.is_some());
                let err = res.unwrap();
                assert_eq!(err, TransactionExecutionError::UtxoAlreadySpent)
            }
        }

        mod with_insufficient_utxos {
            use crate::week_2::utxo_model::{
                tests::transaction::{from_address, to_address},
                Transaction, TransactionExecutionError, Txo,
            };

            #[test]
            fn should_return_insufficient_inputs() {
                // Arrange
                let mut transaction = Transaction::new(
                    vec![
                        Txo::new(String::from(from_address), 5),
                        Txo::new(String::from(from_address), 7),
                    ],
                    vec![Txo::new(String::from(to_address), 22)],
                );

                // Act
                let res = transaction.execute();

                // Assert
                assert!(res.is_some());
                let err = res.unwrap();
                assert_eq!(err, TransactionExecutionError::InsufficientInputs)
            }

            #[test]
            fn should_not_mark_inputs_as_spent() {
                // Arrange
                let mut transaction = Transaction::new(
                    vec![
                        Txo::new(String::from(from_address), 5),
                        Txo::new(String::from(from_address), 4),
                    ],
                    vec![Txo::new(String::from(to_address), 10)],
                );

                // Act
                let res = transaction.execute();

                // Assert
                assert!(res.is_some());
                let err = res.unwrap();
                assert_eq!(err, TransactionExecutionError::InsufficientInputs);

                transaction
                    .input_utxos
                    .iter()
                    .for_each(|txo| assert!(!txo.spent));
            }
        }
    }
}
