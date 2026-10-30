mod console;
mod tokenizer;
use console::io;
use tokenizer::pretokenizer;
use tokenizer::frequencies;

fn main() {
    let raw_input = io::get_input();
    // let input = pretokenizer::pretokenize(&raw_input);

    let wf = frequencies::get_words_freq(&[raw_input.as_str()]);
    let pf = frequencies::get_pairs_freq(&wf);
    
    // вывод частоты пар символов (чисто для проверочки)
    for (pieces, count) in &pf {
        let word: String = pieces.iter().collect();
        println!("  {:>8} → {}", word, count);
    }
}
