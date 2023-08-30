fn twelve_days_of_christmas(day: u8) {
    let gifts = match day {
        1 => "a partridge in a pear tree",
        2 => "two turtle doves",
        3 => "three french hens",
        4 => "four calling birds",
        5 => "five gold rings",
        6 => "six geese a-laying",
        7 => "seven swans a-swimming",
        8 => "eight maids a-milking",
        9 => "nine ladies dancing",
        10 => "ten lords a-leaping",
        11 => "eleven pipers piping",
        12 => "twelve drummers drumming",
        _ => panic!("Invalid day: {}", day),
    };

    let mut line = format!("On the {} day of Christmas, my true love gave to me:", day);

    for i in 1..(day + 1) {
        if i == 1 {
            line.push_str(&gifts);
        } else {
            line.push_str(", ");
            line.push_str(&gifts);
        }
    }

    println!("{}\n", line);
}

fn main() {
    for day in 1..13 {
        twelve_days_of_christmas(day);
    }
}