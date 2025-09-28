/**Функция для вычисления площади треугольника*/
pub fn calculate_triangle_area(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> f64 {
    ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
}

/**Функция для вычисления площади поверхности пирамиды*/
pub fn calculate_pyramid_surface(area: f64, height: f64) -> f64 {
    area * height
}

/**Функция для расчета координат третьих точек треугольника по координатам двух других точек и площади*/
pub fn find_third_vertex(x1: f64, y1: f64, x2: f64, y2: f64, area: f64) -> ((f64, f64), (f64, f64)) {
    // Вычисляем разницу по координатам
    let delta_x = x2 - x1;
    let delta_y = y2 - y1;
    
    // Вычисляем возможные значения для координат третьей точки (x3, y3)
    let factor = (2.0 * area) / (delta_x.powi(2) + delta_y.powi(2)).sqrt();

    // Два возможных решения для y3
    let x3_1 = x1 + delta_y * factor;
    let y3_1 = y1 - delta_x * factor;

    let x3_2 = x1 - delta_y * factor;
    let y3_2 = y1 + delta_x * factor;

    // Возвращаем два возможных варианта координат
    ((x3_1, y3_1), (x3_2, y3_2))
}

