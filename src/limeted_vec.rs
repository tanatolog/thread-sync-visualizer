/** Вектор с ограниченной длинной */
pub struct LimitedVec<T> {
    vec: Vec<T>,
    max_size: usize,
}

impl<T> LimitedVec<T> {
    /** Конструктор LimitedVec с максимальной длиной */
    pub fn new(max_size: usize) -> Self {
        Self {  // создаем структуру и возвращаем ссылку на нее
            vec: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /** Добавляем элемент, только если не превышен max_size */
    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.vec.len() < self.max_size {
            self.vec.push(item);
            Ok(())
        } else {
            Err("The maximum vector length has been exceeded")
        }
    }

    /** Метод для получения текущей длины вектора*/
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /** Метод для проверки, является ли вектор пустым*/
    pub fn is_empty(&self) -> bool { 
        self.vec.is_empty()
    }

    /**Метод удаления элемента по индексу*/
    pub fn remove(&mut self, index: usize) -> Result<T, &'static str> {
        if index < self.vec.len() {
            Ok(self.vec.remove(index))
        } else {
            Err("The maximum vector index has been exceeded")
        }
    }

/* 
     /** Метод для получения элемента по индексу*/
     pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }   

    /** Метод для удаления элемента с конца вектора */
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
*/
}