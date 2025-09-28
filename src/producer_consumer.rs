
/**Задача о производителе и потребителе */
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::shared_container::SharedContainer;
use crate::limeted_vec::LimitedVec;


pub const TASK_DEKKER: &str = "Задача о производителе и потребителе с кольцевым буфером решена с помощью алгоритма Деккера.";

pub const BUFFER_DEKKER_SIZE: usize = 6;
const TASK_DEKKER_SIZE: usize = 100;

/**Функция управления скоростью потоков*/
fn delay(speed: Arc<Mutex<usize>>) {
    let time;
    if *speed.lock().unwrap() == 0 {
        loop {
            thread::sleep(Duration::from_millis(500));
            if *speed.lock().unwrap() != 0 {
                break;
            }
        }
        time = 2000 - (*speed.lock().unwrap() as u64 * 20);
    } else {
        time = 2000 - (*speed.lock().unwrap() as u64 * 20);
    }
    thread::sleep(Duration::from_millis(time));
}

/**Функция производителя*/
pub fn producer_thread(shared_buffer: Arc<SharedContainer<LimitedVec<(usize, usize)>>>, output_mutex: Arc<Mutex<Vec<String>>>, speed: Arc<Mutex<usize>>) {
    for i in 1..=TASK_DEKKER_SIZE {
        let length: usize;
        delay(speed.clone());

        {
            let lock = &*shared_buffer;  // разыменоваем arc и получаем ссылку на кортеж SharedContainer<LimitedVec<f64>>
            let mut buffer = lock.get_first();    // ожидание освобождения ресурса и его захват

            while buffer.len() == BUFFER_DEKKER_SIZE {                           // если в буфере нет места, то освобождаем рес и ждем
                lock.unlock_first();
                thread::sleep(Duration::from_millis(100));
                buffer = lock.get_first();
            }

            length = buffer.len();
            buffer.push((i, length)).unwrap();
            lock.unlock_first();                                          // Освобождение реса
        }                                       

        let entry = format!("Положил продукт №{i} в ячейку {length}");
        {
            output_mutex.lock().unwrap().push(entry);                     // ожидание освобождения мьютекса и его захват
            // Освобождение мьютекса
        }
    }
}

/**Функция для потребителя*/
pub fn consumer_thread(shared_buffer: Arc<SharedContainer<LimitedVec<(usize, usize)>>>, output_mutex: Arc<Mutex<Vec<String>>>, speed: Arc<Mutex<usize>>) {
    for _i in 1..=TASK_DEKKER_SIZE {
        let product: (usize, usize);
        let length: usize;
        delay(speed.clone());

        {
            let lock = &*shared_buffer;  // разыменоваем arc и получаем ссылку на кортеж SharedContainer<LimitedVec<f64>>
            let mut buffer = lock.get_second();   // ожидание освобождения ресурса и его захват
    
            while buffer.is_empty() {                                     // если буфер пуст, то освобождаем рес и ждем
                    lock.unlock_second();
                    thread::sleep(Duration::from_millis(100));
                    buffer = lock.get_second();
            }

            product = buffer.remove(0).unwrap();
            length = buffer.len();
            lock.unlock_second();                                         // Освобождение реса
        }

        let number = product.0;
        let cell = product.1;
        let entry = format!("Получил продукт №{number} из ячейки {cell}, в буфере {length}");

        {
            output_mutex.lock().unwrap().push(entry);
            // Освобождение мьютекса
        }
    }
}