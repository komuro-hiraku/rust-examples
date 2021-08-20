use std::thread;
use std::time::Duration;


fn generate_workout(intensity: u32, random_number: u32) {

    let expensive_closure = | num | {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", // 腕立て
            expensive_closure(intensity)
        );

        println!(
            "Next, do {} situps!",  // 腹筋
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!(
                "Take a break today! Remember to stay hydrated!"
            );
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
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
