fn quize_one() {
    let mut x = 0;
    'a: loop {
         x += 1;
         'b: loop {
             if x > 10 {
                 continue 'a;
             } else {
                 break 'b;
             }
        }
        break;
    }
}

fn quize_two() {
    let a = [6.9; 10];
    let mut store = 0.0;
    for dot in a {
        store += dot;
    }
    println!("total= {}", store);
}

// homework-1_1
fn fahrenheit_to_celsius(f: f32) -> f32 {
    let term1: f32 = f - 32.0;
    let term2: f32 = 5.0/9.0;

    let c:f32 = term1*term2;

    return c;
}

// homework-1_2
fn celsius_to_fahrenheit(c: f32) -> f32 {
  let term1: f32 = 9.0/5.0;
  let term2: f32 = c * term1;
  let f: f32 = term2 + 32.0;

  return f;
}

// homework 2_1
fn n_th_fibonacci_number(n: i32) -> i32 {
  let mut f: i32 = 0;
  if n == 0 {
    return 0;
  } else if n == 1 {
    return 1;
  } else {
    f += n_th_fibonacci_number(n-1) + n_th_fibonacci_number(n-2); // eqn: f_n = f_{n-1} + f_{n-2};
    return f;
  }
}

// home_work 3_1
fn print_gifts(day: u32) {
  let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "And a partridge in a pear tree",
    ];

    let start_idx = 12 - day as usize;
    for i in start_idx..12 {
      if i == 1 && i == 11 {
        println!("A partridge in a pear tree");
      } else {
        println!("{}", gifts[i]);
      }
    }
}

fn print_day_intro(day: u32) {
  let day_word = match day {
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
    _ => unreachable!(),
  };
  println!("On the {} day of Christmas, my true love gave to me:", day_word);
}

fn main() {
  // quizes
  quize_one();
  quize_two();

  // homework
  let f:f32 = 86.0;
  println!("{}", fahrenheit_to_celsius(f));
  let c:f32 = fahrenheit_to_celsius(f);
  println!("{}", celsius_to_fahrenheit(c));
  let n: i32 = 7;
  println!("{}", n_th_fibonacci_number(n));
  
  
  println!("The Twelve Days of Christmas\n");
  
  for day in 1..12 {
    print_day_intro(day);
    print_gifts(day);
    println!();
  }
}
