/**Модуль для описания приложения */
use eframe::egui;
use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Condvar};

use crate::limeted_vec::LimitedVec;
use crate::shared_container::SharedContainer;
use crate::mutex_sync::{TASK_MUTEX, BUFFER_MUTEX_SIZE, first_thread, second_thread};
use crate::producer_consumer::{TASK_DEKKER, BUFFER_DEKKER_SIZE, producer_thread, consumer_thread};

/**Структура приложения */
pub struct App {
    first_speed_ref: Arc<Mutex<usize>>,
    second_speed_ref: Arc<Mutex<usize>>,

    first_table_mutex_ref: Arc<Mutex<Vec<String>>>,
    second_table_mutex_ref: Arc<Mutex<Vec<String>>>,

    first_handle: Option<JoinHandle<()>>,
    second_handle: Option<JoinHandle<()>>,

    producer_speed_ref: Arc<Mutex<usize>>,
    consumer_speed_ref: Arc<Mutex<usize>>,

    producer_table_mutex_ref: Arc<Mutex<Vec<String>>>,
    consumer_table_mutex_ref: Arc<Mutex<Vec<String>>>,

    producer_handle: Option<JoinHandle<()>>,
    consumer_handle: Option<JoinHandle<()>>,

    current_page: usize,
    exit: Arc<AtomicUsize>,
}

/**Деструктор приложения */
impl Drop for App {
    fn drop(&mut self) {
        self.execute_threads();
        self.execute_producer_consumer();
    }
}

/**Реализация интерфейса формы */
impl eframe::App for App {
    /**Функция обновления окна */
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { 
        ctx.request_repaint();                                      // перерисовать интерфейс

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {  // Верхняя панель с переключением страниц
            ui.horizontal(|ui| {
                if ui.button("Мьютекс").clicked() { self.current_page = 0; }
                if ui.button("Деккер").clicked() { self.current_page = 1; }
            });
        });

        match self.current_page {
            0 => self.show_page0(ctx),                              // первая страница
            1 => self.show_page1(ctx),                              // вторая страница
            _ => {}
        }
    }
}

/**Методы класса приложения */
impl App {
    /**Конструктор приложения */
    pub fn new() -> Self {
        // Создаем мьютексы, оборачиваем в arc для разделения доступа к ним
        let first_table_mutex_ref = Arc::new(Mutex::new(Vec::new()));
        let second_table_mutex_ref = Arc::new(Mutex::new(Vec::new()));
        let first_speed_ref = Arc::new(Mutex::new(0));
        let second_speed_ref = Arc::new(Mutex::new(0));

        let producer_table_mutex_ref = Arc::new(Mutex::new(Vec::new()));
        let consumer_table_mutex_ref = Arc::new(Mutex::new(Vec::new()));
        let producer_speed_ref = Arc::new(Mutex::new(0));
        let consumer_speed_ref = Arc::new(Mutex::new(0));

        let mut slf = Self { // создаем поля
            first_speed_ref,
            second_speed_ref,

            first_handle: None,
            second_handle: None,

            first_table_mutex_ref,
            second_table_mutex_ref,

            producer_speed_ref,
            consumer_speed_ref,

            producer_handle: None,
            consumer_handle: None,

            producer_table_mutex_ref,
            consumer_table_mutex_ref,
            
            current_page: 0,
            exit: Arc::new(AtomicUsize::new(0)),
        };

        slf.birth_threads();
        slf.birth_producer_consumer();
        slf                       // возвращаем ссылку на созданный экземпляр приложения
    }

    /**Отрисовка таблиц */
    fn draw_table(&self, ui: &mut egui::Ui, data_ref: &Arc<Mutex<Vec<String>>>, _table_name: &str, title: &str) {
        let data = data_ref.lock().unwrap();
        ui.vertical(|ui| {
            ui.label(title);
            egui::Frame::none()
                .stroke(egui::Stroke::new(2.0, egui::Color32::GRAY))
                .inner_margin(egui::style::Margin::same(5.0))
                .show(ui, |ui| {
                    egui::ScrollArea::new([false, true]).show(ui, |ui| {
                        for entry in data.iter() {
                            ui.label(entry);
                        }
                    });
                });
        });
    }

    /**Инициализация и запуск потоков */
    fn birth_threads(&mut self) {
        let buffer_mutex_ref = Arc::new((Mutex::new(LimitedVec::new(BUFFER_MUTEX_SIZE)), Condvar::new()));
        let first_output_mutex = self.first_table_mutex_ref.clone();
        let second_output_mutex = self.second_table_mutex_ref.clone();
        let first_speed_mutex = self.first_speed_ref.clone();
        let second_speed_mutex = self.second_speed_ref.clone();
        // скорость = 0
        *self.first_speed_ref.lock().unwrap() = 0;
        *self.second_speed_ref.lock().unwrap() = 0;

        // Клонируем указатели на мьютексы для передачи и создаем потоки
        self.first_handle = Some(thread::spawn({
            let buffer_mutex = buffer_mutex_ref.clone();
            move || {
                first_thread(buffer_mutex, first_output_mutex, first_speed_mutex);
            }
        }));

        self.second_handle = Some(thread::spawn({
            let buffer_mutex = buffer_mutex_ref;
            move || {
                second_thread(buffer_mutex, second_output_mutex, second_speed_mutex);
            }
        }));

    }

    /**Инициализация и запуск потребителя и производителя */
    fn birth_producer_consumer(&mut self) {
        let buffer_mutex_ref = Arc::new(SharedContainer::new(LimitedVec::new(BUFFER_DEKKER_SIZE)));
        let producer_output_mutex = self.producer_table_mutex_ref.clone();
        let consumer_output_mutex = self.consumer_table_mutex_ref.clone();
        let producer_speed_mutex = self.producer_speed_ref.clone();
        let consumer_speed_mutex = self.consumer_speed_ref.clone();
        // скорость = 0
        *self.producer_speed_ref.lock().unwrap() = 0;
        *self.consumer_speed_ref.lock().unwrap() = 0;

        // Клонируем указатели на мьютексы для передачи и создаем потоки
        self.producer_handle = Some(thread::spawn({
            let buffer_mutex = buffer_mutex_ref.clone();
            move || {
                producer_thread(buffer_mutex, producer_output_mutex, producer_speed_mutex);
            }
        }));

        self.consumer_handle = Some(thread::spawn({
            let buffer_mutex = buffer_mutex_ref;
            move || {
                consumer_thread(buffer_mutex, consumer_output_mutex, consumer_speed_mutex);
            }
        }));

    }

    /**Остановка потоков */
    fn execute_threads(&mut self) {
        *self.first_speed_ref.lock().unwrap() = 100;
        *self.second_speed_ref.lock().unwrap() = 100;
        self.exit.store(1, Ordering::SeqCst);            // сигнал потокам завершить работу

        // Ожидаем завершения потоков
        if let Some(handle) = self.first_handle.take() {
            handle.join().unwrap();
        }
        if let Some(handle) = self.second_handle.take() {
            handle.join().unwrap();
        }
    }

    /**Перезапуск потоков и очистка таблиц*/
    fn reset_threads(&mut self) {
        self.execute_threads();

        self.first_table_mutex_ref.lock().unwrap().clear();         // Очистка таблиц
        self.second_table_mutex_ref.lock().unwrap().clear();

        self.exit.store(0, Ordering::SeqCst);            // Сброс флага завершения

        self.birth_threads();
    }

    /**Остановка производителя и потребителя */
    fn execute_producer_consumer(&mut self) {
        *self.producer_speed_ref.lock().unwrap() = 100;
        *self.consumer_speed_ref.lock().unwrap() = 100;
        self.exit.store(1, Ordering::SeqCst);            // сигнал потокам завершить работу

        // Ожидаем завершения потоков
        if let Some(handle) = self.producer_handle.take() {
            handle.join().unwrap();
        }
        if let Some(handle) = self.consumer_handle.take() {
            handle.join().unwrap();
        }
    }

    /**Перезапуск производителя и потребителя и очистка таблиц*/
    fn reset_producer_consumer(&mut self) {
        self.execute_producer_consumer();

        self.producer_table_mutex_ref.lock().unwrap().clear();         // Очистка таблиц
        self.consumer_table_mutex_ref.lock().unwrap().clear();

        self.exit.store(0, Ordering::SeqCst);            // Сброс флага завершения

        self.birth_producer_consumer();
    }

    /**Отобрисовка 0 страницы */
    fn show_page0(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right")                // правая боковая панель
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Скорость первого потока:");
            ui.add(egui::Slider::new(&mut *self.first_speed_ref.lock().unwrap(), 0..=100));
            ui.label("Скорость второго потока:");
            ui.add(egui::Slider::new(&mut *self.second_speed_ref.lock().unwrap(), 0..=100));

            if ui.button("Сброс").clicked() {
                self.reset_threads();
            }
        });

        egui::CentralPanel::default()                   // центральная панель
        .show(ctx, |_| {

            egui::TopBottomPanel::top("central")  // побочная верхняя панель
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(TASK_MUTEX);
            });

            egui::SidePanel::left("table1")            // побочная левая панель
                .min_width(150.0)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_table(ui, &self.first_table_mutex_ref, "table1", "Первый поток");
                });

            egui::SidePanel::right("table2")            // побочная правая панель
                .min_width(680.0)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_table(ui, &self.second_table_mutex_ref, "table2", "Второй поток");
                });
        });
    }

    /**Отобрисовка 1 страницы */
    fn show_page1(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right")                // правая боковая панель
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Скорость производителя:");
            ui.add(egui::Slider::new(&mut *self.producer_speed_ref.lock().unwrap(), 0..=100));
            ui.label("Скорость потребителя:");
            ui.add(egui::Slider::new(&mut *self.consumer_speed_ref.lock().unwrap(), 0..=100));

            if ui.button("Сброс").clicked() {
                self.reset_producer_consumer();
            }
        });

        egui::CentralPanel::default()                   // центральная панель
        .show(ctx, |_| {

            egui::TopBottomPanel::top("central")  // побочная верхняя панель
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(TASK_DEKKER);
            });

            egui::SidePanel::left("table1")            // побочная левая панель
                .min_width(300.0)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_table(ui, &self.producer_table_mutex_ref, "table1", "Производитель");
                });

            egui::SidePanel::right("table2")            // побочная правая панель
                .min_width(300.0)
                .resizable(false)
                .show(ctx, |ui| {
                    self.draw_table(ui, &self.consumer_table_mutex_ref, "table2", "Потребитель");
                });
        });
    }

}
