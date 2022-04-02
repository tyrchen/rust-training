use v8_live::JsRuntime;

fn main() {
    JsRuntime::init();
    JsRuntime::init();
    // new isolate
    let mut runtime = JsRuntime::new(None);
    let code = r#"
        async function hello() {
            let result = print({a: 1, b: 2});
            print(result);
            return await fetch("https://www.rust-lang.org/");
        }
        let result = await hello();
        print(result);
    "#;
    let result = runtime.execute_script(code, true);
    println!("Result is: {:?}", result);
}
