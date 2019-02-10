use Box;
use Vec;


struct Node<T, U, V> {
    func: Box<T>;
    params: Vec<Box<U>>;
    result: Box<V>;
}