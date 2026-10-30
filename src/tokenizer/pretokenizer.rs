// [derive...] - макрос для автогенерации реализации трейтов Debug, Clone, Copy, PartialEq, Eq
// CharClass - перечисление, описывающие используемые классы символов
// (буква, цифра, знак пунктуации или разделитель)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharClass {
    Letter,
    Digit,
    Punct,
    Space,
}

fn get_char_class(c: char) -> CharClass {
    if c.is_whitespace() {
        // is_whitespace даст true на '\n' '\r', tab, пробел и другие пробельные символы.
        return CharClass::Space;
    }
    if c.is_ascii_digit() {
        // только арабские цифры
        return CharClass::Digit;
    }
    if c.is_alphabetic() {
        // всё считается буквой в латинице, кириллице и др. Uncode алфавитах.
        return CharClass::Letter;
    }
    // если др. варианты не подошли, считаем символ пункуацией: , . ? ! : ; ' " { @ # и т.д. - любые спецсимволы,
    // а также эмодзи
    CharClass::Punct
}

// функция, разбивающая исходную строку на подстроки (слова, числа, спецсимволы)
// к каждому "слову" будет прибавляться символ
pub fn pretokenize(text: &str) -> Vec<String> {
    let mut result = Vec::new();              // набор слов, возвращаемый функцией
    let mut current = String::new();          // фрагмент исходной строки. Постепенно заполняется сиволами текущего слова.
    let mut current_class = CharClass::Space; // класс последнего добавленного символа (и всех символов текущего слова)
    let mut at_word_start = true;             // флаг, отвечающий на вопрос: мы в начале нового слова?

    for c in text.chars() {
        let c_class = get_char_class(c); // получаем класс текущего символа

        // проверяем, является ли текущий символ пробельным (пробел, табуляция и т.д.)
        if c_class == CharClass::Space {
            // если да, проверяем содержимое current
            if !current.is_empty() {
                // если current НЕ пуст, значит до пробельного символа было какое-то слово
                // тогда заносим это слово (т.е. current) в result
                result.push(std::mem::take(&mut current));
            }
            // отмечаем начало нового слова и завершаем итерацию
            at_word_start = true;
            continue;
        }

        // делаем тоже самое, что и с пробелами, для пунктуации, только сама пунктуация тоже идёт в result
        if c_class == CharClass::Punct {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }
            result.push(c.to_string());
            at_word_start = true;
            continue;
        }

        // если след. символ отличается по классу от предыдущих, завершаем текущее слово и заносим его в result
        if c_class != current_class && !current.is_empty() {
            result.push(std::mem::take(&mut current));
            at_word_start = true;
        }

        // если текущий символ - это начало нового слова, то добавляем спецсимвол '▁'
        // этот символ будет нужен на этапе BPE токенизации, когда мы будем большие слова дробить на части
        if at_word_start {
            current.push('▁');
            at_word_start = false;
        }

        current.push(c);         // записываем символ в подстроку (current)
        current_class = c_class; // сохраняем класс символов текущего слова
    }

    // если символ был последним, но последнее слово ещё не попало в result,
    // то считаем слово (current) завершённым (т.е. больше символов не последует) и отправляем его в result.
    if !current.is_empty() {
        result.push(current);
    }
    result
}
