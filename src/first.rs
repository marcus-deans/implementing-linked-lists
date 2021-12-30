//pub allows use of List outside this module
pub enum List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}
//Box provides heap allocation -> ownership for allocation and drop contenets when out of scope

//enum declares type containing one of several values
// struct declraes type with many values simultaneously

struct Node {
    elem: i32,
    next: List,
}