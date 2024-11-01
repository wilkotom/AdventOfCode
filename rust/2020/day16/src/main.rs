use std::fs;
use std::collections::HashMap;


#[derive(Debug)]
struct Bound {
    min: usize,
    max: usize
}

#[derive(Debug)]
struct Restriction {
    lower: Bound,
    upper: Bound
}

fn main() {
    let (fields, my_ticket, other_tickets) = parse_data(String::from("./input.txt"));
    let (valid_tickets, error_rate) = part1(&fields, &other_tickets);
    println!("Part 1 answer: {}", error_rate);
    let correct_mappings = number_ticket_fields(&fields, &valid_tickets);
    let mut final_answer = 1;
    for heading in correct_mappings.keys() {
        if heading.len() >= 9 && &heading[0..9] == "departure"{
            let field_number = correct_mappings.get(heading).unwrap().get(0).unwrap();
            final_answer *= my_ticket.get(*field_number).unwrap();
        }
    }
    println!("Part 2 answer: {}", final_answer);
}

fn part1(ticket_fields: &HashMap<String,Restriction>, other_tickets: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, usize) {
    let mut error_rate = 0;
    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
    for ticket in other_tickets.iter() {
        let mut valid_ticket = true;
        for column in ticket.iter() {
            let mut valid_column = false;
            for ticket_field in ticket_fields.keys() {
                if ticket_fields[ticket_field].lower.min <= *column && *column  <= ticket_fields[ticket_field].lower.max ||
                ticket_fields[ticket_field].upper.min <= *column && *column <= ticket_fields[ticket_field].upper.max {
                    valid_column = true;
                }
            }
            if !valid_column {
                error_rate += column;
                valid_ticket = false
            }
        }
        if valid_ticket{
            valid_tickets.push(ticket.clone())
        }
    }
    (valid_tickets, error_rate)
}

fn number_ticket_fields(ticket_fields: &HashMap<String,Restriction>, other_tickets: &Vec<Vec<usize>>) -> HashMap<String, Vec<usize>> {
    let mut ticket_mapping: HashMap<String, Vec<usize>> = HashMap::new();
    for ticket_field in ticket_fields.keys() {

        ticket_mapping.insert(ticket_field.to_string(), Vec::new() );
    }

    for i in 0..other_tickets[0].len() {
        for ticket_field in  ticket_fields.keys() {
            let mut possible_match = true;
            for ticket in other_tickets.iter() {
                // println!("Checking if {} is valid for rule {}: {:?}", ticket[i], ticket_field, ticket_fields[ticket_field]);
                if ticket[i] < ticket_fields[ticket_field].lower.min || ticket[i] > ticket_fields[ticket_field].upper.max ||
                ticket[i] > ticket_fields[ticket_field].lower.max && ticket[i] < ticket_fields[ticket_field].upper.min {
                    possible_match = false;
                } 
            }
            if possible_match {
                ticket_mapping.get_mut(ticket_field).unwrap().push(i);
            }
        }

    }


    for _ in 0..ticket_fields.len() {
        for ticket_field in ticket_fields.keys() {
            let possible_fields = ticket_mapping.get(ticket_field).unwrap().clone();
            if possible_fields.len() == 1 {
                for other_field in ticket_fields.keys().filter(|&f| f != ticket_field) {
                    ticket_mapping.get_mut(other_field).unwrap().retain(|&x| x!= possible_fields[0]);
                }
            }
        }
    }

    ticket_mapping
}

fn parse_data(filename: String) -> (HashMap<String,Restriction>, Vec<usize>, Vec<Vec<usize>>) {
    let raw_data = fs::read_to_string(filename).unwrap_or("".to_string());
    let sections: Vec<_> = raw_data.split("\n\n").collect();
    let mut ticket_fields: HashMap<String,Restriction> = HashMap::new();
    for field in sections[0].split("\n") {
            
            let field_name: String = field.split(": ").collect::<Vec<_>>()[0].to_string();
            let bounds: Vec<_> = field.split(": ").collect::<Vec<_>>()[1].split(" ").collect();
            let lower = Bound{min: bounds[0].split("-").collect::<Vec<_>>()[0].parse::<usize>().unwrap_or(0),
                              max: bounds[0].split("-").collect::<Vec<_>>()[1].parse::<usize>().unwrap_or(0)};
            let upper = Bound{min: bounds[2].split("-").collect::<Vec<_>>()[0].parse::<usize>().unwrap_or(0),
                              max: bounds[2].split("-").collect::<Vec<_>>()[1].parse::<usize>().unwrap_or(0)};
            ticket_fields.insert(field_name, Restriction{lower,upper});
        }
    let my_ticket: Vec<_> = sections[1].split("\n").collect::<Vec<_>>()[1].split(",").map(|x| x.parse::<usize>().unwrap_or(0)).collect();
    let mut other_tickets: Vec<Vec<usize>> = Vec::new();
    for line in &sections[2].split("\n").collect::<Vec<_>>()[1..] {
        other_tickets.push(line.split(",").map(|x| x.parse::<usize>().unwrap_or(0)).collect());
    }
    (ticket_fields, my_ticket, other_tickets)
}