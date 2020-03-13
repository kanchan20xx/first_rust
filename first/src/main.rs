// coding at utf-8
fn main() {
    // Display Hello, world!
    println!("Hello, world!");
    println!("You should input number!");
    // let : immutable 変数を作る.
    // mut : mutable変数に昇格
    // String型の変数作成。new()はstatic関数。(と思う。)
    let mut input = String::new();
    // 標準入力から、一行分読み取り -> inputに格納。
    // & は参照
    // expectはio::Resultオブジェクトのメソッド。
    // io::ResultがErrのときに、引数のメッセージを表示。
    std::io::stdin().read_line(&mut input).expect("Fail");
    println! ("Your input is {}", input);
}
