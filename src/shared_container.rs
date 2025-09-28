use std::cell::UnsafeCell;

use crate::dekker::Dekker;

/**Контейнер для доступа к общему ресурсу, работает только с 2 потоками */
pub struct SharedContainer<T> {
    data: UnsafeCell<T>, // оборачиваем данные в UnsafeCell, чтобы изменять их при наличии неизменяемой ссылки
    dek_lock: Dekker,
}

impl<T> SharedContainer<T> {
    /**Конструктор */
    pub fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            dek_lock: Dekker::new(),
        }
    }
    
    /**Заблокировать ресурс и получить первую изменяемую ссылку*/
    pub fn get_first(&self) -> &mut T {
        self.dek_lock.lock(0);

        unsafe { &mut *self.data.get() }
    }

    /**Заблокировать ресурс и получить вторую изменяемую ссылку*/
    pub fn get_second(&self) -> &mut T {
        self.dek_lock.lock(1);

        unsafe { &mut *self.data.get() }
    }

    /**1 поток разблокирует ресурс*/
    pub fn unlock_first(&self) {
        self.dek_lock.unlock(0);
    }

     /**2 поток разблокирует ресурс*/
    pub fn unlock_second(&self) {
        self.dek_lock.unlock(1);
    }
}

unsafe impl<T> Sync for SharedContainer<T> {}
