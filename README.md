# Broken App - Rust Diagnostic Project

Этот проект создан для демонстрации и исправления различных проблем в Rust-коде:

- Undefined Behavior (UB)
- Data races
- Memory leaks
- Use-after-free
- Производительность и оптимизации

## Структура проекта

broken-app/
├── Cargo.toml # Конфигурация проекта
├── src/
│ ├── lib.rs # Основная логика с багами
│ ├── algo.rs # Низкопроизводительные алгоритмы
│ ├── concurrency.rs # Многопоточный код с data races
│ └── bin/
│ └── demo.rs # Демонстрационное приложение
├── tests/
│ └── integration.rs # Интеграционные тесты
├── benches/
│ ├── baseline.rs # Бенчмарки для сравнения
│ └── criterion.rs # Criterion бенчмарки
├── scripts/ # Вспомогательные скрипты
└── artifacts/ # Результаты диагностики

## Инструменты которые использовались для оптимизации кода

- **Miri** - обнаружение Undefined Behavior
- **AddressSanitizer (ASan)** - обнаружение проблем с памятью
- **ThreadSanitizer (TSan)** - обнаружение data races
- **perf + FlameGraph** - профилирование производительности
- **Criterion** - бенчмарки

## Команды которые использовались для диагностики

```bash
# Запуск всех тестов с Miri
cargo +nightly miri test

# Запуск конкретного теста
cargo +nightly miri test --test integration

# Запуск с подробным backtrace
MIRIFLAGS="-Zmiri-backtrace=full" cargo +nightly miri test

# Запуск без игнорирования утечек
MIRIFLAGS="-Zmiri-ignore-leaks=no" cargo +nightly miri test

# Запуск с ASan (требуется пересборка стандартной библиотеки)
RUSTFLAGS="-Z sanitizer=address" \
cargo +nightly test --target x86_64-unknown-linux-gnu \
  -Z build-std --target x86_64-unknown-linux-gnu

# Запуск только тестов (без док-тестов)
RUSTFLAGS="-Z sanitizer=address" \
cargo +nightly test --tests --target x86_64-unknown-linux-gnu \
  -Z build-std --target x86_64-unknown-linux-gnu

# С обнаружением утечек
RUSTFLAGS="-Z sanitizer=address" \
ASAN_OPTIONS="detect_leaks=1" \
cargo +nightly test --target x86_64-unknown-linux-gnu \
  -Z build-std --target x86_64-unknown-linux-gnu

# Запуск с TSan
RUSTFLAGS="-Z sanitizer=thread" \
cargo +nightly test --target x86_64-unknown-linux-gnu \
  -Z build-std --target x86_64-unknown-linux-gnu

# Запуск только тестов (без док-тестов)
RUSTFLAGS="-Z sanitizer=thread" \
cargo +nightly test --tests --target x86_64-unknown-linux-gnu \
  -Z build-std --target x86_64-unknown-linux-gnu

# Сборка с отладочными символами
RUSTFLAGS="-g -C debuginfo=2" cargo build --release --bin demo_extreme

# Запись профиля (требуются права root)
sudo perf record -g --call-graph dwarf -F 9999 -o artifacts/perf.data ./target/release/demo_extreme

# Просмотр отчёта
sudo perf report -i artifacts/perf.data --call-graph

# Генерация flamegraph
git clone https://github.com/brendangregg/FlameGraph.git /tmp/FlameGraph
sudo perf script -i artifacts/perf.data | \
    /tmp/FlameGraph/stackcollapse-perf.pl | \
    /tmp/FlameGraph/flamegraph.pl > artifacts/flamegraph.svg 

# Запуск baseline бенчмарков с сохранением результатов
cargo bench --bench baseline > artifacts/baseline_before.txt 2>&1

# Запуск после оптимизаций
cargo clean && cargo bench --bench baseline > artifacts/baseline_after.txt 2>&1

```
