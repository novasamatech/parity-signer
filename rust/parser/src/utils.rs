pub fn path_to_string(path: Vec<String>) -> String {
    path.join(" >> ")
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
    (type_name == "AccountId32") || (type_name == "AccountId20")
}

pub fn field_type_name_is_call(type_name: &str) -> bool {
    (type_name == "Call") || (type_name == "RuntimeCall")
}
