use std::io::read_to_string;

fn main() {
    println!("########################################");
    println!(
        "Hactuss Rust repository for way too simple rust programs and way too many println macros"
    );
    println!("What would you like to run? (Type name or number)");
    println!("1 | Marcov Chain");
    println!("2 | Center Calc");

    let mut select = String::new();
    let final_select = std::io::stdin().read_line(&mut select).unwrap();

    println!("Value final: {final_select}");
    let next = final_select - 1;
    match next {
        1 => markov_chain(0.9, 0.1, 0.7, 0.3),
        2 => _center_calc(1920, 1080, 200, 200),
        _ => println!("Not found"),
    }
}
fn markov_chain(a_self: f32, a_out: f32, b_self: f32, b_out: f32) {
    println!("########################################");
    if a_self + a_out != 1.0 {
        println!("Markov chain invalid!");
        println!("{a_self} + {a_out} have to be equal to 1")
    } else if b_self + b_out != 1.0 {
        println!("Markov chain invalid!");
        println!("{b_self} + {b_out} have to be equal to 1")
    } else {
        println!("Markov chain valid!")
    }
    println!("----------------------------------------");
    println!("--{a_self}--> (A) ---{a_out}--->");
    println!("          <---{b_out}--- (B) <--{b_self}--");
    println!("########################################");
}
fn _center_calc(canvas_width: i32, canvas_height: i32, item_width: i32, item_height: i32) {
    println!("###### Center Calculator 1.0 ######");
    println!("(c) Hactuss 2026");

    let center_x: i32 = { canvas_width / 2 - item_width / 2 };
    let center_y: i32 = { canvas_height / 2 - item_height / 2 };

    println!("{center_x}");
    println!("{center_y}");
}
