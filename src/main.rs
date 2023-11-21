fn main() {
    let f_to_c = fahrenheit_to_celsius(95.0);
    let c_to_f = celsius_to_fahrenheit(36.0);
    println!("Fahrenheit to celsius: {f_to_c}");
    println!("Celsius to Fahrenheit: {c_to_f}");
    let fibo = fibonacci(5);
    println!("The result of fibonacci is: {fibo}");
    christmas_song();
}
fn fibonacci(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    for i in 2..n {
        let temp = b;
        b = b + a;
        a = temp;
    }
    b
}
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn christmas_song(){
   let christmas_days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleven", "twelth"];
   let things = [
       "Twelve drummers drumming",
       "Eleven pipers piping",
       "Ten lords a-leaping",
       "Nine ladies dancing",
       "Eight maids a-milking",
       "Seven swans a-swimming",
       "Six geese a-laying",
       "Five golden rings",
       "Four calling birds",
       "Three french hens",
       "Two turtle doves and",
       "A partridge in a pear tgree",
   ];
   for i in 0..12 {
       println!("On the {} day of Christmas, my true love sent to me", christmas_days[i]);
       for j in 0..i+1 {
           println!("{}", things[11-j]);
       }
   }
}

fn loop_examples() {
    //  let mut count = 0;
    //  'counting_up : loop {
    //      println!("count = {count}");
    //      let mut remaining = 10;
    //      loop {
    //          println!("remaining = {remaining}");
    //          if remaining == 9 {
    //              break;
    //          }
    //          if count == 2 {
    //              break 'counting_up;
    //          }
    //          remaining -= 1;
    //      }
    //      count += 1;
    //  }
    //  println!("End count = {count}");

    // let mut number = 3;
    // while number > 0 {
    //    println!("The value of number is {number}");
    //    number -= 1;
    // }
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("For: the value is: {element}");
    }

    for index_ in 0..5 {
        println!("the index is {index_} the value is: {}", a[index_]);
    }

    for index_ in (0..=4).rev() {
        println!(
            "Reversed: the index is {index_} the value is: {}",
            a[index_]
        );
    }
}
