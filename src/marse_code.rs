use std::collections::HashMap;

pub fn get_morse_codes() -> HashMap<char, &'static str> {
    let mut morse_code_table = HashMap::new();
    morse_code_table.insert('a', ".-");
    morse_code_table.insert('b', "-...");
    morse_code_table.insert('c', "-.-.");
    morse_code_table.insert('d', "-..");
    morse_code_table.insert('e', ".");
    morse_code_table.insert('f', "..-.");
    morse_code_table.insert('g', "--.");
    morse_code_table.insert('h', "....");
    morse_code_table.insert('i', "..");
    morse_code_table.insert('j', ".---");
    morse_code_table.insert('k', "-.-");
    morse_code_table.insert('l', ".-..");
    morse_code_table.insert('m', "--");
    morse_code_table.insert('n', "-.");
    morse_code_table.insert('o', "---");
    morse_code_table.insert('p', ".--.");
    morse_code_table.insert('q', "--.-");
    morse_code_table.insert('r', ".-.");
    morse_code_table.insert('s', "...");
    morse_code_table.insert('t', "-");
    morse_code_table.insert('u', "..-");
    morse_code_table.insert('v', "...-");
    morse_code_table.insert('w', ".--");
    morse_code_table.insert('x', "-..-");
    morse_code_table.insert('y', "-.--");
    morse_code_table.insert('z', "--..");
    morse_code_table
}
