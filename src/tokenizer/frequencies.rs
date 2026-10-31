use std::collections::BTreeMap;

use crate::tokenizer::pretokenizer::pretokenize;
use crate::tokenizer::Pieces;

// функция-счётчик слов в тексте
// на вход принимает "корпус" (набор текстов)
// и возвращает таблицу, сопоставляющую "претокены" с их частотой в данных текстах
// "претокены" - это слова, числа, спецсимволы и т.д. - короч всё, что выдаёт претокенизатор (функция pretokenize)
pub fn get_words_freq(texts: &[&str]) -> BTreeMap<Pieces, usize> {
    let mut frequencies: BTreeMap<Pieces, usize> = BTreeMap::new();

    // берём один из текстов
    for text in texts {
        // берём одно слово из выбранного текста, полученное из претокенизатора
        for word in pretokenize(text) {
            // крамсаем слово на отдельные символы
            let chars: Pieces = word.chars().collect();

            // добавляем последовательность символов в таблицу
            // если последовательность уже была, просто крутим счётчик инкрементом (+1)
            *frequencies.entry(chars).or_insert(0) += 1;
        }
    }

    frequencies
}

// функция-счётчик пар символов для BPE токенизатора
// на вход принимает табличку частотности слов в текстах изз frequencies
// на выходе в pairs_counts выдаёт такую же BTreeMap табличку, только с частотами пар символов, а не слов
pub fn get_pairs_freq(word_freq: &BTreeMap<Pieces, usize>) -> BTreeMap<(char, char), usize> {
    let mut pairs_freq: BTreeMap<(char, char), usize> = BTreeMap::new();

    // берём слово (как массим символов) и его частоту в тексте
    for (pieces, &freq) in word_freq {
        // берём кусочек из 2-х символлов из слова (перебераем так, пока не кончится слово)
        for window in pieces.windows(2) 
        {
            // берём эту пару символов как картеж
            let pair = (window[0], window[1]);
            // кладём эту пару символов в табличку ДЛЯ ПАР символов
            // если пара уже есть в таблице, увеличиваем её частоту на freq
            // где freq - это частота в ТЕКСТЕ выбранного СЛОВА, из которого мы выдрали текущую пару символов
            *pairs_freq.entry(pair).or_insert(0) += freq; 
        }
    }

    pairs_freq
}

// функция поиска самой частой пары (если их несколько, берёт первую из таблицы)
fn find_best_pair(pairs_freq: &BTreeMap<(char, char), usize>) -> Option<((char, char), usize)> {
    // смотри на первую пару. Если None - функция вернёт None,
    // если что-то есть, запишет в кортеж (first_pair, first_freq) и пойдём дальше
    let (&first_pair, &first_freq) = pairs_freq.first_key_value()?;

    // записываем кортеж пара+частота в best (первый в таблице pairs_freq)
    let mut best = (first_pair, first_freq);

    // пробегаемся по pairs_freq таблице
    for (&pair, &freq) in pairs_freq {
        // если кортеж в best по частоте меньше, чем текущая рассматриваемая пара,
        // то вносим новую (более частую) пару в best
        if freq > best.1 {
            best = (pair, freq);
        }
    }

    // возвращаем самую частую пару (если было несколько, то самую первую, которая попалась)
    Some(best)
}
