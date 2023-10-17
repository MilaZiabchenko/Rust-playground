use rand::prelude::*;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
enum SwissMountain {
    SanSalvatore,
    Pilatus,
    Matterhorn,
}

impl SwissMountain {
    fn as_str(&self) -> &str {
        match self {
            SwissMountain::SanSalvatore => "San Salvatore",
            SwissMountain::Pilatus => "Pilatus",
            SwissMountain::Matterhorn => "Matterhorn",
        }
    }
}

#[derive(Clone)]
struct Traveler {
    name: String,
    city: String,
}

impl Traveler {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_name(&mut self, name: String) {
        self.name = name;
    }

    fn new(name: &str, city: &str) -> Traveler {
        Traveler {
            name: String::from(name),
            city: String::from(city),
        }
    }
}

fn get_first_word(string: &str) -> &str {
    for (index, &item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &string[..index];
        }
    }

    &string
}

fn get_second_word(string: &str) -> Result<&str, String> {
    let mut end = string.len();
    let mut space_indexes = Vec::new();

    for (index, &item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            space_indexes.push(index);
        }

        if space_indexes.len() == 2 {
            end = space_indexes[1];

            break;
        }
    }

    if space_indexes.len() == 0 {
        Err("There is just one word here.".to_string())
    } else {
        Ok(&string[space_indexes[0] + 1..end])
    }
}

fn print_moonwalker_message(name: &str) {
    let moonwalkers = fs::read_to_string("src/moonwalkers.txt").unwrap();

    for moonwalker in moonwalkers.lines() {
        if get_first_word(moonwalker) == name {
            println!("\n(By the way, your name-fellow {moonwalker} walked on the Moon ✨)");

            return;
        }
    }

    println!(
        "\n(By the way, a person named {} has not yet been to the Moon 😇)",
        name
    );
}

fn get_user_name_from_input_and_save_it_to_the_file() -> String {
    let mut user_input = String::new();

    println!("\nPlease, enter your name > ");

    io::stdin().read_line(&mut user_input).unwrap();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("src/user_name.txt")
        .expect("It should open the file");

    file.write_all(&user_input.trim().as_bytes())
        .expect("It should write to the file");

    let file_contents = fs::read_to_string("src/user_name.txt").expect("It should read the file");

    file_contents
}

fn read_user_name_from_input_saved_to_the_file_and_print_greeting() {
    let file_contents = get_user_name_from_input_and_save_it_to_the_file();
    let name_from_file = file_contents.trim();

    println!("\nHello, {name_from_file}! 😊");

    print_moonwalker_message(name_from_file);
}

fn celsius_to_fahrenheit(celsius_temp: f32) -> f32 {
    1.8 * celsius_temp + 32.0
}

fn calc_money_for_journey(days: u16, amount_per_day: u16, extra_money: u16) -> Result<u16, String> {
    if days == 0 {
        Err("Cannot go on a zero-day journey!".to_string())
    } else if amount_per_day == 0 && extra_money == 0 {
        Err("Cannot travel without money!".to_string())
    } else {
        Ok(days * amount_per_day + extra_money)
    }
}

fn trim_leading_and_trailing_spaces(string: &str) -> &str {
    let mut start = 0;
    let mut end = 0;

    for (index, character) in string.chars().enumerate() {
        if character != ' ' {
            start = index;

            break;
        }
    }

    for (index, character) in string.chars().rev().enumerate() {
        if character != ' ' {
            end = string.len() - index;

            break;
        }
    }

    &string[start..end]
}

fn get_alt_name(name: &str, additional_name: &str) -> String {
    let mut alt_name = String::from(name);

    alt_name.push_str(additional_name);

    alt_name
}

fn get_backpack_weight_per_traveler(
    total_weight: f64,
    number_of_travelers: f64,
) -> Result<f64, String> {
    if number_of_travelers < 1.0 {
        Err("Invalid number of travelers".to_string())
    } else {
        Ok(total_weight / number_of_travelers)
    }
}

fn print_backpacks_description(backpacks_info: (u8, &str, &str, &str)) {
    let (quantity, color, qualifier, adjective) = backpacks_info;

    println!("Our {quantity} {color} backpacks look {qualifier} {adjective}! 😎");
}

fn main() {
    let mut countdown: u8 = 5;

    let result = loop {
        if countdown == 3 {
            break countdown;
        }
        println!("{countdown}");

        countdown -= 1;
    };

    println!("{result}");

    while countdown > 1 {
        countdown -= 1;

        println!("{countdown}");
    }

    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let flag = args[1].trim();
            let name_from_cli_args = args[2].trim();

            if flag == "--name" {
                println!("\nHello, {name_from_cli_args}! 😊");

                print_moonwalker_message(name_from_cli_args);
            } else {
                read_user_name_from_input_saved_to_the_file_and_print_greeting();
            }
        }
        _ => {
            read_user_name_from_input_saved_to_the_file_and_print_greeting();
        }
    }

    // variable assignment with short-circuiting operation (right-hand operator is not evaluated)
    let summer = true || panic!();
    let winter = false && panic!();

    const SUN_SYMBOL: char = '\u{1f506}';
    const PRIMARY_ACTIVITY: &str = "traveling";

    let activities = if summer {
        "hiking & swimming"
    } else if winter {
        "skating & skiing"
    } else {
        "hiking & biking"
    };

    if summer {
        println!("\nI write these lines in summer {SUN_SYMBOL} — nice time for {PRIMARY_ACTIVITY}, {activities}!",);
    } else if winter {
        println!(
            "\nI write these lines in winter — good time for {PRIMARY_ACTIVITY}, {activities}!",
        );
    } else {
        println!(
            "\nIt's neither too cold nor hot — great time for {PRIMARY_ACTIVITY}, {activities}!",
        );
    }

    let waypoints = [
        ["Vinnytsia", "Ukraine"],
        ["Zurich", "Switzerland"],
        ["Turin", "Italy"],
    ];

    println!("\nThese are the waypoints of my upcoming journey:\n");

    for (index, [city, country]) in waypoints.iter().enumerate() {
        let formatted_waypoint = format!("{}, {}", city, country);

        println!("{}. {}", index + 1, formatted_waypoint);
    }

    println!("\nDuring my journey I am going to visit the following countries:\n");

    let mut countries_to_visit = Vec::new();
    let [my_native_city, my_country] = waypoints[0];

    for [_city, country] in waypoints {
        if country != my_country {
            let country_to_visit = country;

            println!("- {country_to_visit}");

            countries_to_visit.push(country_to_visit);
        }
    }

    assert_eq!(countries_to_visit[0], waypoints[1][1]);
    assert_eq!(countries_to_visit[1], waypoints[2][1]);

    let starting_point: &str = &format!("{}, {}", my_native_city, my_country);

    assert_eq!(starting_point, "Vinnytsia, Ukraine");

    println!("\nThe journey kicks off in {starting_point}.");

    let celsius_temp: f32 = 37.0;
    let fahrenheit_temp: f32 = celsius_to_fahrenheit(celsius_temp);

    println!("As of August 2023, it's really hot in {my_native_city} — the temperature rises up to around {celsius_temp}° C, which is about {fahrenheit_temp:.0}° F, in case some American citizens are reading this story... 😄");

    println!(
        "\nI travel to {} and {} 😊",
        countries_to_visit[0], countries_to_visit[1]
    );

    let destination = waypoints[1][0];
    let mut destination_message = String::from(format!("First I fly to {destination}; "));

    let destination = waypoints[2][0]; // variable shadowing
    destination_message.push_str(&format!("my final destination will be {destination}."));

    println!("{destination_message}");

    let san_salvatore = SwissMountain::SanSalvatore.as_str();

    println!(
        "When in {}, I am gonna climb three of its magnificent mountains: {}, {:?}, and {:?}.",
        countries_to_visit[0],
        san_salvatore,
        SwissMountain::Pilatus,
        SwissMountain::Matterhorn
    );

    let mut travel_days = vec![
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
    ];

    travel_days.push("Saturday");

    let number_of_days: u16 = travel_days.len() as u16; // type casting
    let money_for_journey: Result<u16, String> = calc_money_for_journey(number_of_days, 100, 500);

    match money_for_journey {
        Ok(money) => println!("\nSetting off with {} euros in my pocket...", money),
        Err(error) => println!("Error: {}", error),
    }

    // binary notation and bitwise operations

    // clearing (x AND 0) and setting (x OR 1) value of a specific a bit, when assigning value to a variable
    let number_of_travelers = 0b0000_1010u8 & 0b1111_0111 | 0b0000_0010;

    match number_of_travelers {
        0 => println!("No one is traveling at the moment..."),
        1 => println!("I am traveling solo... 🐾"),
        2 => println!("A good friend of mine is traveling together with me... 🐾🐾"),
        _ => println!("I am traveling with a gang of friends... 😀🙃😀"),
    }

    assert_eq!(number_of_travelers, 2);

    // checking a bit value at a position (x AND 1), when reading the value of a variable
    println!(
        "(Now bit 1 is {0:08b}, bit 3 is {1:08b}, the binary number of travelers is {2:08b}, so the decimal number of travelers is {2}...)",
        number_of_travelers & 0b0000_0010,
        number_of_travelers & 0b0000_1000,
        number_of_travelers
    );

    let me = Traveler::new("Mila", my_native_city);
    let mut also_me = Traveler { ..me.clone() };
    let my_best_friend = Traveler {
        name: String::from("Leo"),
        ..me.clone()
    };

    also_me.update_name(String::from("Mi"));
    also_me.city = String::from("Rome");

    let my_name = me.get_name();
    let my_short_name = also_me.get_name();

    assert_eq!(my_name, "Mila");
    assert_eq!(my_short_name, "Mi");

    let Traveler { name, city } = &my_best_friend;
    let my_best_friend_name = name;
    let my_best_friend_city = city;

    assert_eq!(my_best_friend_name, "Leo");
    assert_eq!(my_best_friend_city, "Vinnytsia");

    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    assert_eq!(sparkle_heart, "💖");

    println!(
        "\nI am {my_name}, — or just {my_short_name}, and {my_best_friend_name} is my best friend {sparkle_heart}"
    );
    println!(
        "I like to walk the streets of {}, and {} prefers to stay at home in {} 😊",
        also_me.city, my_best_friend_name, my_best_friend_city
    );

    let delicious_synonym = trim_leading_and_trailing_spaces("  yummy 😋   ");

    assert_eq!(delicious_synonym, "yummy 😋");

    println!("I often call my dear friend {}", delicious_synonym);

    let my_best_friend_alt_name = get_alt_name(&my_best_friend_name, " King");

    println!("Sometimes I call him {} 😎", my_best_friend_alt_name);
    println!(
        "{} — or {} if you wish — is tender and loyal 💙",
        my_best_friend_name, my_best_friend_alt_name
    );

    let first_word = get_first_word(&my_best_friend_alt_name);
    let second_word = get_second_word(&my_best_friend_alt_name);

    match &second_word {
        Ok(second_word) => println!(
            "I call my awesome friend {first_word} {second_word} because he is truly magnificent and gorgeous 😇",
        ),
        Err(error) => println!("Error: {error}"),
    }

    let last_word = my_best_friend_alt_name.split(" ").last().unwrap();

    assert_eq!(second_word, Ok(last_word));

    let mut packing_list = Vec::new();

    packing_list.push("documents");
    packing_list.push("clothes");
    packing_list.push("toiletries");

    print!("\nWe're traveling for {number_of_days} days with backpacks full of stuff: ");
    for item in packing_list {
        print!("{item}, ")
    }
    print!("and other useful things...\n");

    let backpacks_info_values: (u8, &str, &str, &str) =
        (number_of_travelers, "red", "pretty", "fancy");

    assert_eq!(backpacks_info_values.0, 2);

    print_backpacks_description(backpacks_info_values);

    let backpack_weight = get_backpack_weight_per_traveler(7.75, number_of_travelers as f64);

    match backpack_weight {
        Ok(weight) => println!(
            "The {:.1} kg 🎒 is really heavy when you have to do a lot of hiking in the Alps... 😅",
            weight
        ),
        Err(error) => println!("Error: {}", error),
    }

    let any_travel_day = thread_rng().gen_range(0..travel_days.len());
    let the_most_exiting_day = travel_days[any_travel_day];

    println!(
        "\n{} was the most exiting day of our journey, but all good things must come to an end, and we'll be back in {} soon 😉", the_most_exiting_day, city
    );
}
