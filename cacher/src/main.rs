
struct Cacher<F, T> {
    value: Option<T>,
    calculation: F,
}

impl<F, T> Cacher<F, T>
where
    F: Fn(T) -> T,
{
    fn new(calculation: F) -> Cacher<F, T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value (&mut self, arg : T) -> &T {
        self.value.get_or_insert_with(|| (self.calculation)(arg))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| -> u32 {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    };

    let mut expensive_closure = Cacher::new(expensive_closure);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    println!("Hello, world!");
}
