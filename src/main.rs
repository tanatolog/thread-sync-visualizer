use eframe::egui;

mod app;
mod limeted_vec;
mod model;
mod mutex_sync;
mod producer_consumer;
mod dekker;
mod shared_container;

use app::App;

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {             // настройки окна приложения
        default_theme: eframe::Theme::Light,
        initial_window_size: Some(egui::vec2(1030.0, 750.0)),   // Устанавливаем начальный размер
        ..Default::default()                                         // остальные настройки беруться по умолчанию
    };

    eframe::run_native(                                              // запуск приложения
        "Синхронизация потоков",
        options,
        Box::new(|_| { Box::new(App::new()) }),         // приложение будет иметь вид App
    )
}