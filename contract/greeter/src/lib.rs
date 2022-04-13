use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::LookupMap;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: LookupMap<AccountId, String>,
}

impl Default for Welcome {
    fn default() -> Self {
      Self {
        records: LookupMap::new(b"a".to_vec())
      }
    }
}


#[near_bindgen]
impl Welcome {
    /// Change method: set the greeting for signer account. 
    pub fn set_greeting(&mut self, message: String) {
      let account_id = env::signer_account_id();

      env::log_str(
        format!(
          "Hello, {} from {}",
          message,
          account_id
        ).as_str(),
      );

      self.records.insert(&account_id, &message);
    }


    /// View method: get the greeting for a particular account. 
    pub fn get_greeting(&self, account_id: AccountId) -> String {
      match self.records.get(&account_id) {
        Some(greeting) => greeting,
        None => "".to_owned(),
      }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{VMContext, testing_env};
    use near_sdk::test_utils::{VMContextBuilder};


    fn get_context(is_view: bool) -> VMContext {
      VMContextBuilder::new()
          .current_account_id("alice_near".parse().unwrap())
          .signer_account_id("bob_near".parse().unwrap())
          .predecessor_account_id("carol_near".parse().unwrap())
          .is_view(is_view)
          .build()
    }

    #[test]
    fn test_set_and_get_greeting() {
      let context = get_context(false);
      testing_env!(context);

      let mut contract = Welcome::default();
      contract.set_greeting("howdy".to_owned());

      assert_eq!(
        "howdy".to_owned(),
        contract.get_greeting("bob_near".parse().unwrap())
      );
    }

    #[test]
    fn get_default_greeting() {
      let context = get_context(true);
      testing_env!(context);

      let contract = Welcome::default();

      assert_eq!(
        "".to_owned(),
        contract.get_greeting("francis_near".parse().unwrap())
      );
    }
}
