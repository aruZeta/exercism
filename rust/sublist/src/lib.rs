#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_is_sublist = is_sublist(_first_list, _second_list);
    let second_is_sublist = is_sublist(_second_list, _first_list);

    if first_is_sublist && second_is_sublist { Comparison::Equal }
    else if first_is_sublist { Comparison::Sublist }
    else if second_is_sublist { Comparison::Superlist }
    else { Comparison::Unequal }
}

fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.len() == 0 { return true; }

    for i in 0.._second_list.len() {
        if _second_list[i] == _first_list[0] {
            let mut equals = true;
            for j in 1.._first_list.len() {
                if _second_list.len() <= i + j { return false }
                if _first_list[j] != _second_list[i + j] {
                    equals = false;
                    break;
                }
            }
            if equals { return true; }
        }
    }
    return false;
}
