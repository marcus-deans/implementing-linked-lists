use std::mem;

pub struct List<T> {
    head: Link<T>,
}

// use type alias to save on typing
    //prior Link enum was just Option -> instead use Option's methods
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

//since made definitions generic -> also need to implement generically
impl<T> List<T> {
    //don't write List<T> when creating List instance
        //inferred since returning from function expecting List<T>
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            //mem::replace(&mut option, None) very comon
                //method 'take' is the same
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        //match option {None => None, Some(x) => Some(y)} ubiqituous
            //method 'map' is same -> takes function

        //write online with closure -> is anonymous function
        //AND can refer to local variables outside closure
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

