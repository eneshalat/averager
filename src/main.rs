use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut new_args: Vec<i32> = vec![];
    let mut new_arg_push;
    for i in args {
        new_arg_push = i.trim().parse::<i32>().unwrap();
        new_args.push(new_arg_push);
    }
    let mut summary: i32 = 0;
    let mut args_count = 0;
    for a in new_args {
        summary += a;
        args_count += 1;
    }
    let sum_f : f32 = summary as f32;
    let count_f : f32 = args_count as f32;
    let result: f32 = sum_f/count_f;
    println!("result: {}", result);
}
