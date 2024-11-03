fn main() {
    let data = std::fs::read_to_string(String::from("./input.txt")).unwrap();
    let lines = data.split("\n").collect::<Vec<_>>();
    let mut total_tls = 0;
    let mut total_ssl = 0;

    for line in lines {
        let mut supernet: Vec<String> = Vec::new();
        let mut hypernet: Vec<String> = Vec::new();
        let mut accessors: Vec<String> = Vec::new();
        for split in line.split("[") {
            let sections = split.split("]").collect::<Vec<_>>();
            if sections.len() == 1 {
                supernet.push(String::from(sections[0]));
            } else {
                supernet.push(String::from(sections[1]));
                hypernet.push(String::from(sections[0]));
            }
        }
        let mut valid_tls = false;
        for word in &supernet {
            valid_tls = valid_tls || is_abba_compliant(word);
        }
        for word in hypernet {
            valid_tls = valid_tls && !is_abba_compliant(&word);
            accessors.append(&mut get_accessors(&word));
        }
        if valid_tls{
            total_tls += 1;
        }
        println!("{} {}", line, valid_tls);
        println!("{:?}",accessors);
        let mut valid_ssl = false;
        for accessor in accessors {
            for word in &supernet {
                valid_ssl = valid_ssl || word.contains(&accessor) ;
            }
        }
        if valid_ssl{
            total_ssl += 1;
        }

    }
    println!("TLS addresses: {}", total_tls);
    println!("SSL addresses: {}", total_ssl);

}

fn is_abba_compliant(word: &str) -> bool {
    if word.len() < 4 {
        false
    } else {
        let mut compliant = false;
        let word = word.chars().collect::<Vec<_>>();
        for i in 0..word.len() -3 {
            if word[i] == word[i+3] && 
               word[1 + i] == word[2+i] && 
               word[i] != word[i+1] {
                compliant = true;
                break;
            }
        }
        compliant
    }
}

fn get_accessors(network: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    let network = network.chars().collect::<Vec<_>>();
    for i in 0.. network.len()-2 {
        if network[i] == network[i+2] && network[i] != network[i+1] {
            blocks.push(format!("{}{}{}", network[i+1], network[i], network[i+1]));
        }
    }
    blocks
}