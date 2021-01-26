/// Вход: променлива `n`, която описва броя елементи, които ще генерираме в резултата.
///
/// За всяко число от 1 до `n` включително, искаме съответстващия елемент в резултата да е:
///
/// - String със съдържание "Fizz" ако числото се дели на 3, но не на 5
/// - String със съдържание "Buzz" ако числото се дели на 5, но не на 3
/// - String със съдържание "Fizzbuzz" ако числото се дели и на 3, и на 5
/// - Числото конвертирано до низ, във всички други случаи
///
/// Тоест, във `fizzbuzz(3)`, първия елемент ще бъде `String::from("1")`, втория
/// `String::from("2")`, последния `String::from("Fizz")`.
///
/// Ако `n` е 0, очакваме празен вектор за резултат.
///
pub fn fizzbuzz(n: usize) -> Vec<String> {
    let mut res = Vec::<String>::new();

    for elem in 1..n+1 {
        if (elem % 3) == 0 && (elem % 5) != 0 {
            res.push(String::from("Fizz"));
        } else if (elem % 5) == 0 && (elem % 3) != 0 {
            res.push(String::from("Buzz"));
        } else if (elem % 3) == 0 && (elem % 5) == 0 {
            res.push(String::from("Fizzbuzz"));
        } else {
            res.push(elem.to_string());
        }
    }

    return res;
}

/// Вход:
/// - променлива `n`, която описва броя елементи, които ще генерираме в резултата.
/// - променливи `k1` и `k2`, които са двата делителя, които ще използваме за заместване.
///
/// За всяко число от 1 до `n` включително, искаме съответстващия елемент в резултата да е:
///
/// - String със съдържание "Fizz" ако числото се дели на k1, но не на k2
/// - String със съдържание "Buzz" ако числото се дели на k2, но не на k1
/// - String със съдържание "Fizzbuzz" ако числото се дели и на k1, и на k2
/// - Числото конвертирано до низ, във всички други случаи
///
/// Ако `n` е 0, очакваме празен вектор за резултат.
/// Ако `k1` или `k2` са 0 или 1, очакваме функцията да panic-не с каквото съобщение изберете.
///
pub fn custom_buzz(n: usize, k1: u8, k2: u8) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let k1_usize = k1 as usize;
    let k2_usize = k2 as usize;

    for checker in 0..2 {
        if k1_usize == checker || k2_usize == checker {
            panic!("каквото съобщение изберете");
        }
    }

    for elem in 1..n+1 {
        if (elem % k1_usize) == 0 && (elem % k2_usize) != 0 {
            res.push(String::from("Fizz"));
        } else if (elem % k2_usize) == 0 && (elem % k1_usize) != 0 {
            res.push(String::from("Buzz"));
        } else if (elem % k1_usize) == 0 && (elem % k2_usize) == 0 {
            res.push(String::from("Fizzbuzz"));
        } else {
            res.push(elem.to_string());
        }
    }

    return res;
}

/// Параметри:
/// - полета `k1` и `k2`, които са двата делителя, които ще използваме за заместване.
/// - поле `labels`, които са трите етикета, които съответстват на коефициентите
///
pub struct FizzBuzzer {
    pub k1: u8,
    pub k2: u8,
    pub labels: [String; 3],
}

impl FizzBuzzer {
    /// За всяко число от 1 до `n` включително, искаме съответстващия елемент в резултата да е:
    ///
    /// - Първия String от полето `labels` ако числото се дели на k1, но не на k2
    /// - Втория String от полето `labels` ако числото се дели на k2, но не на k1
    /// - Третия String от полето `labels` ако числото се дели и на k1, и на k2
    /// - Числото конвертирано до низ, във всички други случаи
    ///
    /// Ако `n` е 0, очакваме празен вектор за резултат.
    /// Ако `k1` или `k2` са 0 или 1, очакваме функцията да panic-не с каквото съобщение изберете.
    ///
    pub fn take(&self, n: usize) -> Vec<String> {
        let mut res = Vec::<String>::new();
        let k1_usize = self.k1 as usize;
        let k2_usize = self.k2 as usize;

        for checker in 0..2 {
            if k1_usize == checker || k2_usize == checker {
                panic!("каквото съобщение изберете");
            }
        }

        for elem in 1..n+1 {
            if (elem % k1_usize) == 0 && (elem % k2_usize) != 0 {
                res.push(self.labels[0].to_string());
            } else if (elem % k2_usize) == 0 && (elem % k1_usize) != 0 {
                res.push(self.labels[1].to_string());
            } else if (elem % k1_usize) == 0 && (elem % k2_usize) == 0 {
                res.push(self.labels[2].to_string());
            } else {
                res.push(elem.to_string());
            }
        }

        return res;
    }

    /// Параметъра `index` указва кой етикет от полето `labels` променяме, от 0 до 2. Ако подадения
    /// `index` е извън тези рамки, очакваме функцията да panic-не.
    ///
    /// Стойността `value` е низа, който ще сложим на този индекс в полето `labels`.
    ///
    pub fn change_label(&mut self, index: usize, value: &String) {
        if index > 2 {
            panic!("fkfkfkfk segmentation fault. Please provide an index between 0 and 2.");
        }

        self.labels[index] = String::from(value);
    }
}

