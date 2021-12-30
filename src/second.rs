use std::mem;

pub struct List {
    head: Link,
}

// use type alias to save on typing
    //prior Link enum was just Option -> instead use Option's methods
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
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
            Some(node.elem)
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

