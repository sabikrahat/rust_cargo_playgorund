pub fn string_test() -> String {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";

    let mut sentence = String::new();

    if sentence.is_empty() {
        println!("sentence is empty");
    }

    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    let len: usize = sentence.len();
    println!("length of sentence: {}", len);
    println!("{:?}", &sentence[0..5]);
    println!("{:?}", &sentence[8..11]);
    return sentence;
}