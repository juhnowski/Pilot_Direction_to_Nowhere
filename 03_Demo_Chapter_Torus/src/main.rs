// 03_Demo_Chapter_Torus/main.rs

/// Константы геометрического мира станции "Торус-12"
const R_BIG: f64 = 10.0;     // Расстояние от центра бублика до середины трубы
const R_SMALL: f64 = 4.0;    // Радиус самой трубы бублика
const G_MASS: f64 = 150.0;   // Мощность гравитации черной дыры в центре (0, 0, 0)
const STEP_SIZE: f64 = 0.1;  // Шаг дискретизации симуляции лазера

/// Структура трехмерного вектора для локальных пространственных вычислений
#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    /// Вычисление длины (модуля) вектора
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Нормализация вектора (приведение длины к 1.0 без изменения направления)
    fn normalize(&self) -> Vector3 {
        let l = self.length();
        if l == 0.0 {
            Vector3 { x: 0.0, y: 0.0, z: 0.0 }
        } else {
            Vector3 {
                x: self.x / l,
                y: self.y / l,
                z: self.z / l,
            }
        }
    }

    /// Скалярное произведение векторов (Детектор взаимного направления)
    fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Перечисление возможных исходов полета луча в симуляторе
#[derive(Debug, PartialEq)]
enum SimulationResult {
    CapturedByBlackHole, // Луч по спирали засосало в сингулярность
    HitsTorusAgain,      // Траектория пересекла противоположную стену бублика
    LostInNowhere,       // Превышен лимит шагов (луч зациклился или ушел в бесконечность)
}

/// Локальный пошаговый расчет траектории луча в гравитационном поле
fn trace_laser_in_gravitational_field(mut current_pos: Vector3, mut direction: Vector3) -> SimulationResult {
    direction = direction.normalize();

    // Запускаем цикл симуляции, жестко ограниченный 500 шагами для защиты от дедлока
    for step in 0..500 {
        let dist_to_center = current_pos.length();

        // Проверка 1: Если луч подошел слишком близко к сингулярности (горизонту событий) — он поглощен
        if dist_to_center < 1.5 {
            return SimulationResult::CapturedByBlackHole;
        }

        // Проверка 2: Столкновение с внутренней или внешней обшивкой тонуса.
        // Математическое уравнение торуса: (R - sqrt(x^2 + y^2))^2 + z^2 = r^2
        let proj_xy = (current_pos.x.powi(2) + current_pos.y.powi(2)).sqrt();
        let torus_equation = (R_BIG - proj_xy).powi(2) + current_pos.z.powi(2);

        // Пропускаем первые несколько шагов, чтобы луч не задел точку старта на стене
        if step > 5 && torus_equation <= R_SMALL.powi(2) {
            return SimulationResult::HitsTorusAgain;
        }

        // Вычисление гравитационного притяжения к центру (0, 0, 0)
        let gravity_pull = Vector3 {
            x: -current_pos.x / dist_to_center,
            y: -current_pos.y / dist_to_center,
            z: -current_pos.z / dist_to_center,
        };

        // Закон Ньютона: изменение вектора направления под воздействием силы
        let gravity_strength = G_MASS / dist_to_center.powi(2);
        direction.x += gravity_pull.x * gravity_strength * STEP_SIZE;
        direction.y += gravity_pull.y * gravity_strength * STEP_SIZE;
        direction.z += gravity_pull.z * gravity_strength * STEP_SIZE;
        direction = direction.normalize();

        // Смещение луча в пространстве на один шаг вперед
        current_pos.x += direction.x * STEP_SIZE;
        current_pos.y += direction.y * STEP_SIZE;
        current_pos.z += direction.z * STEP_SIZE;
    }

    // Если лимит шагов исчерпан, значит луч попал в пространственную ловушку бесконечного ожидания
    SimulationResult::LostInNowhere
}

fn main() {
    println!("[LOG] Запуск бортовой навигационной системы станции Торус-12...");
    
    // Навигатор стоит на внутренней дуге бублика (10.0 - 4.0 = 6.0)
    let hero_pos = Vector3 { x: 6.0, y: 0.0, z: 0.0 };
    
    // Пытается выстрелить лазером вдоль стены, надеясь попасть на противоположную сторону
    let laser_dir = Vector3 { x: 0.1, y: 1.0, z: 0.0 };

    println!("[LOG] Расчет траектории луча при массе черной дыры G_MASS = {}...", G_MASS);
    let result = trace_laser_in_gravitational_field(hero_pos, laser_dir);

    match result {
        SimulationResult::CapturedByBlackHole => {
            println!("[LOG] Предупреждение: Луч захвачен гравитационным капканом!");
            println!("[LOG] Статус вычислений: Триумф!"); 
            // Триумф означает, что программа успешно и безопасно локализовала тупик, не зависнув
        }
        SimulationResult::HitsTorusAgain => {
            println!("[LOG] Успех: Сигнал прорвался на противоположную сторону станции!");
        }
        SimulationResult::LostInNowhere => {
            println!("[LOG] Ошибка: Луч ушел в бесконечное ожидание («в никуда»)");
        }
    }
}
