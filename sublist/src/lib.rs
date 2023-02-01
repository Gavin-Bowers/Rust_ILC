#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    
    if _first_list == _second_list {
        return Comparison::Equal;
        
    } else if _second_list.len() < _first_list.len() {
        if sublist(_second_list, _first_list) == Comparison::Sublist {
            return Comparison::Superlist;
        }
    } else {
        for i in 0 ..= (_second_list.len() - _first_list.len()) {
            if _second_list[i .. (i + _first_list.len())] == *_first_list {
                return Comparison::Sublist;
            }
        }
    }

    //Default case
    return Comparison::Unequal
}
