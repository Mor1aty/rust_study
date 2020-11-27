// 泛型
// 寻找 slice 中最大的值（i32 和 char）

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_all<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 22, 89, 1010, 23];
    let result = largest_i32(&number_list);
    println!("{}", result);
    assert_eq!(result, 1010);

    let char_list = vec!['a', 'm', 'q'];
    let result = largest_char(&char_list);
    println!("{}", result);
    assert_eq!(result, 'q');

    let result = largest_all(&char_list);
    println!("{}", result);
    let result = largest_all(&number_list);
    println!("{}", result);
}
