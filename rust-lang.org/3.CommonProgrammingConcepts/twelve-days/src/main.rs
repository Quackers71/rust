fn twelve_days_of_christmas(day: u8) {
    let mut gifts = vec![];

    for i in 1..=day {
        match i {
            1 => gifts.push("a partridge in a pear tree"),
            2 => gifts.push("two turtle doves"),
            3 => gifts.push("three french hens"),
            4 => gifts.push("four calling birds"),
            5 => gifts.push("five gold rings"),
            6 => gifts.push("six geese a-laying"),
            7 => gifts.push("seven swans a-swimming"),
            8 => gifts.push("eight maids a-milking"),
            9 => gifts.push("nine ladies dancing"),
            10 => gifts.push("ten lords a-leaping"),
            11 => gifts.push("eleven pipers piping"),
            12 => gifts.push("twelve drummers drumming"),
            _ => (),
        }
    }

    let mut line = format!("On the {} day of Christmas my true love gave to me, ", match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    });

    for (i, gift) in gifts.iter().rev().enumerate() {
        if i == 0 {
            line.push_str(&gift);
        } else {
            line.push_str(", ");
            line.push_str(&gift);
        }
    }

    println!("{}\n", line);
}

fn main() {
    for day in 1..13 {
        twelve_days_of_christmas(day);
    }
}
