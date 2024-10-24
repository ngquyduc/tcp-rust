// DO NOT MODIFY THIS FILE
use rand::{Rng, SeedableRng};

const MAX_IO_DURATION_MS: usize = 2000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TaskType {
    CpuIntensiveTask,
    IOIntensiveTask,
}

impl TaskType {
    pub fn from_u8(value: u8) -> Option<TaskType> {
        match value {
            0 => Some(TaskType::CpuIntensiveTask),
            1 => Some(TaskType::IOIntensiveTask),
            _ => None, // Handle invalid u8 values
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Task;

impl Task {
    #[allow(dead_code)]
    pub fn execute(typ: u8, seed: u64) -> u8 {
        match TaskType::from_u8(typ).unwrap() {
            TaskType::CpuIntensiveTask => Self::do_cpu_intensive_task(seed),
            TaskType::IOIntensiveTask => Self::do_io_intensive_task(seed),
        }
    }

    #[allow(dead_code)]
    pub async fn execute_async(typ: u8, seed: u64) -> u8 {
        match TaskType::from_u8(typ).unwrap() {
            TaskType::CpuIntensiveTask => Self::do_cpu_intensive_task_async(seed).await,
            TaskType::IOIntensiveTask => Self::do_io_intensive_task_async(seed).await,
        }
    }

    const DATA_SIZE: usize = 1024 * 1024;
    const MULTIPLIERS: [usize; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 16, 64, 256];
    const DATA_LIMIT: u64 = u8::MAX as u64;

    fn do_cpu_intensive_task(seed: u64) -> u8 {
        let mut data = [0u8; Self::DATA_SIZE];
        let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(seed);
        let total_rounds = Self::MULTIPLIERS[rng.gen::<usize>() % 16] * 4 * 1024;

        let mut dep = 0u8;

        for _ in 0..total_rounds {
            let index = (rng.gen::<usize>() + dep as usize) % Self::DATA_SIZE;
            data[index] = data[index].wrapping_add(rng.gen::<u8>());
            dep = dep.wrapping_add(data[index]);
        }

        let index = (rng.gen::<usize>() + dep as usize) % Self::DATA_SIZE;
        data[index]
    }

    fn do_io_intensive_task(seed: u64) -> u8 {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
        let duration: usize = rng.gen_range(0..=MAX_IO_DURATION_MS);

        std::thread::sleep(std::time::Duration::from_millis(duration as u64));

        rng.gen::<u8>()
    }

    async fn do_cpu_intensive_task_async(seed: u64) -> u8 {
        let mut data = [0u8; Self::DATA_SIZE];
        let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(seed);
        let total_rounds = Self::MULTIPLIERS[rng.gen::<usize>() % 16] * 4 * 1024;

        let mut dep = 0u8;
        for _ in 0..total_rounds {
            let index = (rng.gen::<usize>() + dep as usize) % Self::DATA_SIZE;
            data[index] = data[index].wrapping_add(rng.gen::<u8>());
            dep = dep.wrapping_add(data[index]);
        }

        let index = (rng.gen::<usize>() + dep as usize) % Self::DATA_SIZE;
        data[index]
    }

    async fn do_io_intensive_task_async(seed: u64) -> u8 {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
        let duration: usize = rng.gen_range(0..=MAX_IO_DURATION_MS);

        tokio::time::sleep(std::time::Duration::from_millis(duration as u64)).await;

        rng.gen::<u8>()
    }
}
