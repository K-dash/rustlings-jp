// `use`や`as`キーワードを使うことで`モジュールのパスや新しい名前を導入できます。

#[allow(dead_code)]
mod delicious_snacks {
    // TODO: 以下の`use`構文を修正後に追加してください。
    use self::fruits::PEAR as pear;
    use self::veggies::CUCUMBER as cucumber;

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits::PEAR,
        delicious_snacks::veggies::CUCUMBER,
    );
}
