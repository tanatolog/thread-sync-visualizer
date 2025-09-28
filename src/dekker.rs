use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/**Алгоритм Деккера для 2 потоков */
pub struct Dekker {
    want: [AtomicBool; 2],
    turn: AtomicUsize,
}

impl Dekker {
    /**Конструктор */
    pub fn new() -> Self {
        Dekker {
            want: [AtomicBool::new(false), AtomicBool::new(false)],
            turn: AtomicUsize::new(0),
        }
    }

    /**Метод для входа в критическую секцию потока с номером id (0 или 1)*/
    pub fn lock(&self, id: usize) {
        let other = 1 - id;                                             // Идентификатор другого потока

        self.want[id].store(true, Ordering::SeqCst);                // Устанавливаем флаг намерения войти в критическую секцию

        while self.want[other].load(Ordering::SeqCst) {                 // Пока другой поток хочет войти в критическую секцию
            if self.turn.load(Ordering::SeqCst) != id {                 // Если право передачи у другого потока, ожидаем
                self.want[id].store(false, Ordering::SeqCst);      // Отказ от попытки

                while self.turn.load(Ordering::SeqCst) != id {}         // Ожидаем, пока не наступит наша очередь

                self.want[id].store(true, Ordering::SeqCst);       // Повторная попытка
            }
        }
    }

    /**Метод для выхода из критической секции*/
    pub fn unlock(&self, id: usize) {
        self.turn.store(1 - id, Ordering::SeqCst);                  // Передача права на критическую секцию другому потоку
        self.want[id].store(false, Ordering::SeqCst);               // Сбрасываем флаг намерения войти в критическую секцию
    }
}

