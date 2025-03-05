
#[derive(Debug)]
enum Errors {
    EmptyString,
    InvalidCharacter(char, usize), // char si pozite tuplu !
    NegativeNumber
}

fn parser_for_unsigned_integer(s: &str) -> Result<u32, Errors> {
    if s.is_empty() {
        return Err(Errors::EmptyString);
    }

    if s.starts_with('-') {
        return Err(Errors::NegativeNumber);
    }

    let mut value: u32 = 0;
    for (index, ch) in s.chars().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            value = value * 10 + digit;
        } else {
            let err = Errors::InvalidCharacter(ch, index);
           // println!("Invalid char error: {:?}", err); 
            return Err(err);
        }
    }

    Ok(value)
}

enum Currency {
    Ron,
    Dollar,
    Euro,
    Pound,
    Bitcoin,
}

struct Transaction {
    amount: f64,
    currency: Currency,
}

fn total_in_ron(vector: &Vec<Transaction>) -> f64 {
    let mut total = 0.0;

    for trans in vector {
        let exchg_rate = match trans.currency{
            Currency::Ron => 1.0,
            Currency::Dollar =>4.4,
            Currency::Euro => 4.1,
            Currency::Pound => 7.5,
            Currency::Bitcoin => 100.0
        };
        total = total + trans.amount * exchg_rate;
    }

    total
}


fn ex1(name: String) {
    println!("sup , {} lala", name);

}

fn ex2(n: u32) {
    for i in 0..n {
        let odd = 2*i + 1;
        println!("{}", odd);
    }
}

fn ex3(vector: &[u32]) ->  Option<u32> {
    for &num in vector {
        if num % 2 == 0 {
            return Some(num)
        }
    }
    None
}

fn ex4(words: &[String]) -> Option<String> {
    for word in words {
        if word.len() > 4 {
            return Some(word.to_string())
        }
       
    } None
}
fn main()
{   
    let param = String::from("Daria");
    ex1(param);
    ex2(10);

    let arr = [1,7,3,7,5,7,8,7];
    let slice = &arr[1..=6]; //fara ultimul cu = => il include si pe ultimul

    match ex3(slice) {
        Some(ceva) => println!("nr par gasit: {}", ceva),
        None => println!("nu am pare!")
    }

    let vector = vec! [
        "ana".to_string(),
        "are".to_string(),
        "mere verzi".to_string(),
    ];

    match ex4(&vector) {
        Some(value) => println!("found string greater than 4: {}", value),
        None => println!("Nope no match"),
    }

    let transactions = vec! [
        Transaction { amount: 10.0, currency: Currency::Ron },
        Transaction { amount: 20.0, currency: Currency::Dollar },
        Transaction { amount: 30.0, currency: Currency::Euro },
        Transaction { amount: 40.0, currency: Currency::Pound },
        Transaction { amount: 0.03, currency: Currency::Bitcoin },
    ];
    let total = total_in_ron(&transactions);
    println!("total amount in RON {}:", total);

    let strings = vec![
        ("255", Ok(255)),
        ("120", Ok(42)),
        ("-55", Err(Errors::NegativeNumber)),
        ("", Err(Errors::EmptyString)),
        ("1b55", Err(Errors::InvalidCharacter('b', 1))),
    ];

    for (input, _expected) in strings {
        let result = parser_for_unsigned_integer(input);
        match result {
            Ok(value) => println!("Parsed this {}: {}", input, value),
            Err(e) => println!("Error parsing this {}: {:?}", input, e),
        }
    }

}