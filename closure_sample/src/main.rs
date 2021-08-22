use std::thread;
use std::time::Duration;


fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", // 腕立て
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!",  // 腹筋
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!(
                "Take a break today! Remember to stay hydrated!"
            );
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    // let n = example_closure(5); // 上で String で推論した Closure なのでu32はダメ 　
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{ 
    calculation: T,
    value: Option<u32>,
}


impl<T> Cacher<T>
    where T: Fn(u32) -> u32 
{ 
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}