# borrow_checker

Владение и заимствование в Rust: восемь маленьких задач (`task1` … `task8`) в одном cargo-workspace.
Каждая задача — отдельный bin-крейт с единственным файлом `src/main.rs`, без внешних зависимостей.
Это часть серии учебных проектов вместе с [FirstSteps](../FirstSteps),
[middle-projects](../middle-projects), [pro-projects](../pro-projects) и [Mini-project](../Mini-project).

## Требования

- Rust **1.85+** (все крейты — `edition = "2024"`)
- Стабильный тулчейн: https://rustup.rs

## Как запускать

Из корня воркспейса (рекомендуется):

```bash
cargo run -p task1      # -p task2, -p task3, … -p task8
```

Или из папки задачи:

```bash
cd task5
cargo run
```

Проверить сразу все задачи:

```bash
cargo check --workspace
```

## Задания

| # | Тема | Что делает код | Что демонстрирует |
|---|------|----------------|-------------------|
| [task1](task1) | Итерация по ссылке | Создаёт `inventory: Vec<String>` (`"Меч"`, `"Лук"`), печатает элементы в цикле, затем весь вектор через `{:?}` | `for` по `inventory.iter()` не забирает владение; вывод `Меч`, `Лук`, `["Меч", "Лук"]` |
| [task2](task2) | Перемещение (move) | `let sword = String::from("Железный меч")`, затем `sell(sword)` — владение уходит в `fn sell(item: String)` | move-семантика `String` (тип не `Copy`); в строке 4 закомментирован вариант с ошибкой `E0382 use of moved value` |
| [task3](task3) | Изменяемое заимствование `&mut` | `struct Player { hp }`, печатает `100`, вызывает `heal(&mut player)`, печатает `120` | `&mut Player`, `let mut`, заимствование заканчивается до следующего `println!` — конфликта нет |
| [task4](task4) | Ссылка и мутация `Vec` | `let first = &inventory[0].clone();`, затем `inventory.push(String::from("Щит"))`, печатает `first` | `.clone()` создаёт независимое значение и разрывает заимствование; `&` здесь — ссылка на временный `String` |
| [task5](task5) | Времена жизни (lifetimes) | `find_item(&inventory, "Меч")` возвращает `Option<&'a String>`, `match` печатает найденное или `"Ничего"` | явный lifetime `fn find_item<'a>(inventory: &'a Vec<String>, search_item: &str) -> Option<&'a String>` |
| [task6](task6) | Разделяемые ссылки `&` | `r1 = &npc`, `r2 = &npc` живут одновременно; печать по `r1.name`, `{:?} r1` и обеих ссылок вместе | несколько иммутабельных заимствований сразу, авторазыменование, `#[derive(Debug)]` |
| [task7](task7) | `clone` вместо ссылки | `Vec<Item>`, `let sword = inventory[0].name.clone();`, затем `push` нового Item, печатает `Sword` | исправленный (компилируемый) аналог task4: заимствование разрывается до мутации |
| [task8](task8) | `str::ends_with` и `assert_eq!` | `solution("abc", "c")` и `solution("abc", "D")` печатают `true` и `false` | `ends_with`, `assert_eq!`; вторая проверка ожидает `true` и **паникует** — это ожидаемое поведение задачи |

## Структура

```
borrow_checker/
├── README.md
├── .gitignore
├── Cargo.toml          workspace: members = ["task1" … "task8"], resolver = "3"
├── Cargo.lock
├── task1/ … task8/
│   ├── Cargo.toml      edition 2024, пустой блок [dependencies]
│   ├── .gitignore      /target
│   ├── src/main.rs     вся задача целиком
│   └── target/         (сборка, в git не попадает)
└── target/             общий target воркспейса
```

Общих и внешних зависимостей нет: и корневой, и локальные `Cargo.lock` содержат только сами пакеты.

## Заметки

- **Все восемь задач компилируются.** Ни одна не демонстрирует ошибку компилятора — некомпилируемые
  варианты оставлены в комментариях: `task2:4` (использование перемещённой строки), `task6:18–20`
  (функция `greet_link` по ссылке), `task8:7` (срез `word[0..ending.len()]`).
- Предупреждения компилятора: `task2` — `unused_mut`; `task6` — `unused_mut` и мёртвые
  `greet`/`greet_mut_link`; `task7` — `struct Player is never constructed`.
- `task8` намеренно завершается паникой ассерта (`panicked at task8/src/main.rs`, exit code `101`):
  второй вызов проверяет `false` против эталона `true`.
- Мелкие шероховатости: в `task5` условие записано «наоборот» (`search_item.contains(item)`), но для
  `"Меч"` это работает; в `task4` остаётся лишняя ссылка `&…clone()` на временное значение.
- `#[allow(...)]`, `unwrap()`, `expect()`, `#[test]` и папки `tests/` в коде нет.
- Задачи раньше существовали как отдельные локальные git-репозитории (у семи из восьми не было ни одного
  коммита). История `task8` сохранена локально в `../borrow_checker-submodules-backup/task8.git`.
