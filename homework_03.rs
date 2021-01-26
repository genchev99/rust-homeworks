/// Проверява че следващия символ във входния низ `input` е точно `target`.
///
/// Ако низа наистина започва с този символ, връща остатъка от низа без него, пакетиран във
/// `Some`. Иначе, връща `None`. Примерно:
///
/// skip_next("(foo", '(') //=> Some("foo")
/// skip_next("(foo", ')') //=> None
/// skip_next("", ')')     //=> None
///
pub fn skip_next(input: &str, target: char) -> Option<&str> {
    if input.is_empty() {
        /* if the string is_empty return None */
        return None;
    }

    return if input.as_bytes()[0] as char == target {
        /* If the first element of the string is matching the target it returns the remaining array */
        Some(&input[1..])
    } else {
        /* Otherwise, returns None */
        None
    }
}

/// Търси следващото срещане на символа `target` в низа `input`. Връща низа до този символ и низа
/// от този символ нататък, в двойка.
///
/// Ако не намери `target`, връща оригиналния низ и празен низ като втори елемент в двойката.
///
/// take_until(" foo/bar ", '/') //=> (" foo", "/bar ")
/// take_until("foobar", '/')    //=> ("foobar", "")
///
pub fn take_until(input: &str, target: char) -> (&str, &str) {
    /* Finds the first matching index */
    let offset = input.to_string().find(target).unwrap_or(input.len());

    return (&input[..offset], &input[offset..]);
}

/// Комбинация от горните две функции -- взема символите до `target` символа, и връща частта преди
/// символа и частта след, без самия символ. Ако символа го няма, връща `None`.
///
/// take_and_skip(" foo/bar ", '/') //=> Some((" foo", "bar "))
/// take_and_skip("foobar", '/')    //=> None
///
pub fn take_and_skip(input: &str, target: char) -> Option<(&str, &str)> {
    let (first, second) = take_until(input, target.clone());
    let skip_next_second = skip_next(second, target.clone());

    return if skip_next_second == None {
        None
    } else {
        Some((first, skip_next_second.unwrap()))
    }
}

#[derive(Debug)]
pub enum CsvError {
    IO(std::io::Error),
    ParseError(String),
    InvalidHeader(String),
    InvalidRow(String),
    InvalidColumn(String),
}

impl From<std::io::Error> for CsvError {
    fn from(error: std::io::Error) -> Self {
        CsvError::IO(error)
    }
}
use std::collections::HashMap;

type Row = HashMap<String, String>;

use std::io::BufRead;

pub struct Csv<R: BufRead> {
    pub columns: Vec<String>,
    reader: R,
    selection: Option<Box<dyn Fn(&Row) -> Result<bool, CsvError>>>,
}

use std::io::Write;

impl<R: BufRead> Csv<R> {
    /// Конструира нова стойност от подадения вход. Третира се като "нещо, от което може да се чете
    /// ред по ред".
    ///
    /// Очакваме да прочетете първия ред от входа и да го обработите като заглавна част ("header").
    /// Това означава, че първия ред би трябвало да включва имена на колони, разделени със
    /// запетайки и може би празни места. Примерно:
    ///
    /// - name, age
    /// - name,age,birth date
    ///
    /// В случай, че има грешка от викане на методи на `reader`, тя би трябвало да е `io::Error`.
    /// върнете `CsvError::IO`, което опакова въпросната грешка.
    ///
    /// Ако първия ред е празен, прочитането ще ви върне 0 байта. Примерно, `read_line` връща
    /// `Ok(0)` в такъв случай. Това означава, че нямаме валиден header -- нито една колона няма,
    /// очакваме грешка `CsvError::InvalidHeader`.
    ///
    /// Ако има дублиране на колони -- две колони с едно и също име -- също върнете
    /// `CsvError::InvalidHeader`.
    ///
    /// Ако всичко е наред, върнете конструирана стойност, на която `columns` е списък с колоните,
    /// в същия ред, в който са подадени, без заобикалящите ги празни символи (използвайте
    /// `.trim()`).
    ///

    pub fn new(mut reader: R) -> Result<Self, CsvError> {
        let mut header_line = String::new();
        let header_size = reader.read_line(&mut header_line)?;

        if header_size == 0 {
            return Err(CsvError::InvalidHeader("InvalidHeader indeed".to_string()));
        }

        let mut headers = vec![];
        loop {
            let (column, rest) = take_until(&header_line, ',');
            let new_col = column.trim().to_string();
            if headers.contains(&new_col) {
                return Err(CsvError::InvalidHeader("InvalidHeader indeed".to_string()));
            }

            headers.push(new_col);

            if rest == "" {
                break;
            }

            header_line = skip_next(rest, ',').unwrap().to_string();
        }

        Ok(Self {columns: headers, reader, selection: None})
    }

    /// Функцията приема следващия ред за обработка и конструира `Row` стойност
    /// (`HashMap<String, String>`) със колоните и съответсващите им стойности на този ред.
    ///
    /// Алгоритъма е горе-долу:
    ///
    /// 1. Изчистете реда с `.trim()`.
    /// 2. Очаквате, че реда ще започне със `"`, иначе връщате грешка.
    /// 3. Прочитате съдържанието от отварящата кавичка до следващата. Това е съдържанието на
    ///    стойността на текущата колона на този ред. Не го чистите от whitespace, просто го
    ///    приемате както е.
    /// 4. Ако не намерите затваряща кавичка, това е грешка.
    /// 5. Запазвате си стойността в един `Row` (`HashMap`) -- ключа е името на текущата колона,
    ///    до която сте стигнали, стойността е това, което току-що изпарсихте.
    /// 6. Ако нямате оставащи колони за обработка и нямате оставащо съдържание от реда, всичко
    ///    е ок. Връщате реда.
    /// 7. Ако нямате оставащи колони, но имате още от реда, или обратното, това е грешка.
    ///
    /// За този процес, помощните функции, които дефинирахме по-горе може да ви свършат работа.
    /// *Може* да използвате вместо тях `.split` по запетайки, но ще имаме поне няколко теста със
    /// вложени запетайки. Бихте могли и с това да се справите иначе, разбира се -- ваш избор.
    ///
    /// Внимавайте с празното пространство преди и след запетайки -- викайте `.trim()` на ключови
    /// места. Всичко в кавички се взема както е, всичко извън тях се чисти от whitespace.
    ///
    /// Всички грешки, които ще връщате, се очаква да бъдат `CsvError::InvalidRow`.
    ///
    pub fn parse_line(&mut self, line: &str) -> Result<Row, CsvError> {
        let mut values = vec![];
        let mut copy_of_line = line.clone();
        loop {
            let left_quote_rest = skip_next(copy_of_line.trim(), '"');
            if left_quote_rest == None {
                return Err(CsvError::InvalidRow("No first quote".to_string()));
            }

            let value_packed = take_and_skip(left_quote_rest.unwrap(), '"');
            if value_packed == None {
                return Err(CsvError::InvalidRow("No second quote".to_string()));
            }
            let (value, rest) = value_packed.unwrap();
            values.push(value);
            let other = skip_next(rest.trim(), ',');
            if other == None {
                if rest.trim() != "" {
                    return Err(CsvError::InvalidRow("No comma after quote".to_string()));
                }
                break;
            }
            copy_of_line = other.unwrap();
        }

        /* map values to keys */
        if values.len() != self.columns.len() {
            return Err(CsvError::InvalidRow("Number of values and columns is different".to_string()));
        }

        let zipped = self.columns.iter().zip(values.iter());
        let mut res: Row = Row::new();

        for (column, value) in zipped {
            res.insert(column.to_string(), value.to_string());
        }

        return Ok(res);
    }

    /// Подадената функция, "callback", се очаква да се запази и да се използва по-късно за
    /// филтриране -- при итерация, само редове, за които се връща `true` се очаква да се извадят.
    ///
    /// Би трябвало `callback` да се вика от `.next()` и от `.write_to()`, вижте описанията на тези
    /// методи за детайли.
    ///
    pub fn apply_selection<F>(&mut self, callback: F)
        where F: Fn(&Row) -> Result<bool, CsvError> + 'static
    {
        self.selection = Some(Box::new(callback));
    }

    /// Извикването на този метод консумира CSV-то и записва филтрираното съдържание в подадената
    /// `Write` стойност. Вижте по-долу за пример и детайли.
    ///
    /// Грешките, които се връщат са грешките, които идват от използваните други методи, плюс
    /// грешките от писане във `writer`-а, опаковани в `CsvError::IO`.
    ///
    /// В зависимост от това как си имплементирате метода, `mut` може би няма да ви трябва за
    /// `self` -- ако имате warning-и, просто го махнете.
    ///
    pub fn write_to<W: Write>(mut self, mut writer: W) -> Result<(), CsvError> {
        let length_of_columns = self.columns.len();

        for (index, column_header) in self.columns.iter().enumerate() {
            writer.write(column_header.as_bytes())?;
            if index != length_of_columns - 1 {
                writer.write(b", ")?;
            }
        }
        writer.write(b"\n")?;

        while let Some(row) = self.next() {
            for (index, column_header) in self.columns.iter().enumerate() {
                writer.write(b"\"")?;
                writer.write(row.as_ref().unwrap().get(column_header).unwrap().as_bytes())?;
                writer.write(b"\"")?;
                if index != length_of_columns - 1 {
                    writer.write(b", ")?;
                }
            }
            writer.write(b"\n")?;
        }

        writer.flush()?;
        return Ok(());
    }
}

impl<R: BufRead> Iterator for Csv<R> {
    type Item = Result<Row, CsvError>;

    /// Итерацията се състои от няколко стъпки:
    ///
    /// 1. Прочитаме следващия ред от входа:
    ///     -> Ако има грешка при четене, връщаме Some(CsvError::IO(...))
    ///     -> Ако успешно се прочетат 0 байта, значи сме на края на входа, и няма какво повече да
    ///        четем -- връщаме `None`
    ///     -> Иначе, имаме успешно прочетен ред, продължаваме напред kobrataaa
    /// 2. Опитваме се да обработим прочетения ред със `parse_line`:
    ///     -> Ако има грешка при парсене, връщаме Some(CsvError-а, който се връща от `parse_line`)
    ///     -> Ако успешно извикаме `parse_line`, вече имаме `Row` стойност.
    /// 3. Проверяваме дали този ред изпълнява условието, запазено от `apply_selection`:
    ///     -> Ако условието върне грешка, връщаме тази грешка опакована във `Some`.
    ///     -> Ако условието върне Ok(false), *не* връщаме този ред, а пробваме следващия (обратно
    ///        към стъпка 1)
    ///     -> При Ok(true), връщаме този ред, опакован във `Some`
    ///
    /// Да, тази функция връща `Option<Result<...>>` :). `Option` защото може да има, може да няма
    /// следващ ред, `Result` защото четенето на реда (от примерно файл) може да не сработи.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut next_line = String::new();
            let len_of_next_line = self.reader.read_line(&mut next_line).unwrap();

            if len_of_next_line == 0 {
                return None;
            }

            let parsed_row = self.parse_line(&next_line).unwrap();

            if self.selection.is_none()
                || (self.selection.as_ref().unwrap())(&parsed_row).is_ok()
                && (self.selection.as_ref().unwrap())(&parsed_row).unwrap() {
                return Some(Ok(parsed_row));
            }
        }
    }
}

