mod task1;
mod task2;
mod utils;

fn main() {
    //env_logger::init();

    aoc_helper::benchmark(task1::run_task);
    aoc_helper::benchmark(task2::run_task);
}
