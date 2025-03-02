pub fn path_to_string<'a>(path: Vec<String>) -> String {
	path.join(" >> ")
}

pub fn path_from_parent<'a>(parent_path: &Option<Vec<String>>, maybe_type_name: &Option<String>) -> String {
  match maybe_type_name {
    Some(type_name) => {
      let new_path = parent_path
        .as_ref()
        .unwrap_or(&vec![])
        .into_iter()
        .chain(std::iter::once(type_name))
        .cloned()
        .collect();
      path_to_string(new_path)
    }
    None => {
      "".to_string()
    }
  }
}

pub fn field_type_name_is_balance(type_name: &str) -> bool {
  (type_name == "Balance")
      || (type_name == "T::Balance")
      || (type_name == "BalanceOf<T>")
      || (type_name == "ExtendedBalance")
      || (type_name == "BalanceOf<T, I>")
      || (type_name == "DepositBalance")
      || (type_name == "PalletBalanceOf<T>")
}

pub fn field_type_name_is_account(type_name: &str) -> bool {
  (type_name == "AccountId")
      || (type_name == "AccountId32")
      || (type_name == "AccountId20")
}

pub fn field_type_name_is_call(type_name: &str) -> bool {
  (type_name == "Call") || (type_name == "RuntimeCall")
}