mod rand;

fn bubble_sort(v: &mut Vec<u64>) {
    let mut length = v.len();
    while length > 1 {
        let mut last_changed = 0;
        for i in 1..length {
            if v[i-1] > v[i] {
                (v[i-1], v[i]) = (v[i], v[i-1]);
                last_changed = i;
            }
        }
        length = last_changed;
    }
}

fn selection_sort(v: &mut Vec<u64>) {
    let length = v.len();
    for cursor in 0..length {
        let mut min = cursor;
        for i in cursor+1..length {
            if v[i] < v[min] {
                min = i;
            }
        }
        (v[cursor], v[min]) = (v[min], v[cursor]);
    }
}

fn insertion_sort(v: &mut Vec<u64>) {
    let length = v.len();
    for cursor in 1..length {
        let mut i = cursor;
        while i > 0 && v[i-1] > v[i] {
            (v[i-1], v[i]) = (v[i], v[i-1]);
            i -= 1;
        }
    }
}


fn main() {
    let mut gen = rand::Rand::new();
    let data: Vec<u64> = (1..1000).map(|_| gen.next()).collect();
    
    let mut sorted_data = data.clone();
    sorted_data.sort();

    for f in [bubble_sort, selection_sort, insertion_sort] {
        let mut f_data = data.clone();
        f(&mut f_data);
        assert_eq!(sorted_data, f_data);   
    }
}
