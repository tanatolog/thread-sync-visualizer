/**Синхронизация потоков с помощью мьютексов */
use crate::model::{calculate_pyramid_surface, calculate_triangle_area, find_third_vertex};
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use std::thread::{self};

use crate::limeted_vec::LimitedVec;

pub const TASK_MUTEX: &str = "С помощью мьютексов организована работа параллельных вычислительных потоков.
Первый поток считает площадь треугольника S, координаты вершин которого (4; 3), (8; 12), (i; j), где i, j = [9; 14], шаг 1, затем считает площадь поверхности пирамиды G с высотой равной (i+j). 
Второй поток считает координаты третьей вершины треугольника с координатами (–2; 6), (2; –4), (x; y) по координатам двух известных вершин и его площади S (определенной первым потоком), далее считает объем пирамиды с высотой h = [4; 10], шаг 1.";

pub const BUFFER_MUTEX_SIZE: usize = 36;

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

/**Функция потока 1*/
pub fn first_thread(buffer_mutex: Arc<(Mutex<LimitedVec<f64>>, Condvar)>, output_mutex: Arc<Mutex<Vec<String>>>, speed: Arc<Mutex<usize>>) {
    let mut counter = 1;
    for i in 9..=14 {
        for j in 9..=14 {

            let area = calculate_triangle_area(4.0, 3.0, 8.0, 12.0, i as f64, j as f64);
            let pyramid_surface = calculate_pyramid_surface(area, (i + j) as f64);
            let entry = format!("{counter}: S = {area}, G = {pyramid_surface}");

            delay(speed.clone());

            {
                let (lock, cvar) = &*buffer_mutex;  // разыменоваем arc и получаем ссылку на кортеж (Mutex<LimitedVec<f64>>, Condvar)
                let mut buffer = lock.lock().unwrap();  // ожидание освобождения мьютекса и его захват

                while buffer.len() == BUFFER_MUTEX_SIZE {                                    // если в буфере нет места, то освобождаем мьютекс и ждем
                    buffer = cvar.wait(buffer).unwrap();
                }

                buffer.push(area).unwrap();
                cvar.notify_one();
                // Освобождение мьютекса
            }

            {
                output_mutex.lock().unwrap().push(entry);                              // ожидание освобождения мьютекса и его захват
                // Освобождение мьютекса
            }
            counter += 1;
        }
    }
}

/**Функция для потока 2*/
pub fn second_thread(buffer_mutex: Arc<(Mutex<LimitedVec<f64>>, Condvar)>, output_mutex: Arc<Mutex<Vec<String>>>, speed: Arc<Mutex<usize>>) {
    let mut counter = 1;
    while counter <= 36 {
        let area: f64;

        delay(speed.clone());

        {
            let (lock, cvar) = &*buffer_mutex;  // разыменоваем arc и получаем ссылку на кортеж (Mutex<LimitedVec<f64>>, Condvar)
            let mut buffer = lock.lock().unwrap();  // ожидание освобождения мьютекса и его захват

            while buffer.is_empty() {                                              // если в буфере нет места, то освобождаем мьютекс и ждем
                buffer = cvar.wait(buffer).unwrap();
            }

            area = buffer.remove(0).unwrap();
            cvar.notify_one();
            // Освобождение мьютекса
        }

        // Нахождение координаты третьей вершины (x3, y3) для треугольника с вершинами (-2, 6), (2, -4) и площадью area.
        let ((x3_1, y3_1), (x3_2, y3_2)) = find_third_vertex(-2.0, 6.0, 2.0, -4.0, area);

        // Рассчитываем объем пирамиды
        let mut volume = Vec::new();
        for h in 4..=10 {
            volume.push(area * h as f64);
        }

        let entry = format!("{counter}: S = {area:.2}, Volume = {volume:?}, Third Vertexes = ({x3_1:.2}; {y3_1:.2}), ({x3_2:.2}; {y3_2:.2})");
        {
            output_mutex.lock().unwrap().push(entry);
            // Освобождение мьютекса
        }
        counter += 1;
    }
}