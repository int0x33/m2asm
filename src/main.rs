use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = 2;
    let text = args[1].to_string();
    let mem_addr = text.chars()
    .enumerate()
    .flat_map(|(i, c)| {
        if i != 0 && i % n == 0 {
            Some(' ')
        } else {
            None
        }
        .into_iter()
        .chain(std::iter::once(c))
    })
    .collect::<String>();
    let final_str = format!("0x{}", mem_addr.replace(" ", ", 0x"));
    println!(".byte {}", final_str );
}
