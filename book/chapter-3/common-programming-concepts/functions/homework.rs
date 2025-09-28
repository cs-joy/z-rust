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

// homework-1
fn fahrenheit_to_celsius(f: f32) -> f32 {
    let term1: f32 = f - 32.0;
    let term2: f32 = 5.0/9.0;

    let c:f32 = term1*term2;

    return c;
}

// homework-2
fn celsius_to_fahrenheit(c: f32) -> f32 {
  let term1: f32 = 9.0/5.0;
  let term2: f32 = c * term1;
  let f: f32 = term2 + 32.0;

  return f;
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
}
