fn main() {
    let (tx, rv) = std::sync::mpsc::channel::<i32>(); // Создаем mpsc канал для передачи данных типа i32

    let handle = std::thread::spawn(move || {
        // Создаем поток, передаем в него отправитель
        for i in 0..10 {
            tx.send(i).unwrap(); // на каждой итерации цикла записываем в канал значение
        }
    });

    handle.join().unwrap(); // Ожидаем завершения работы потока

    for i in rv.iter() {
        // Итерируемся по значениям записанным в канал и выводим их
        println!("{i:?}");
    }
}
