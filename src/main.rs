fn main() {
    markov_chain(0.9, 0.1, 0.7, 0.3);
}
fn markov_chain(a_self: f32, a_out: f32, b_self: f32, b_out: f32) {
    println!("########################################");
    if a_self + a_out != 1.0 {
        println!("markov chain incorrect");
        println!("{a_self} + {a_out} have to equal 1")
    } else if b_self + b_out != 1.0 {
        println!("markov chain incorrect");
        println!("{b_self} + {b_out} have to equal 1")
    } else {
        println!("markov chain correct!")
    }
    println!("----------------------------------------");
    println!("--{a_self}--> (A) ---{a_out}--->");
    println!("          <---{b_out}--- (B) <--{b_self}--");
    println!("########################################");
}
