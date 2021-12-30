use std::mem;
//put mem in local scope
//mem::replace -> take value out of borrow by replacing with another value

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


//non-static function = methods -> use self = no declared type
//fn foo(self, arg2: Type 2) -> ReturnType { ... }
//self can be 'self'= Value, '&mut self'=mutable reference, '&self'=shared reference
    //value is true ownership -> new location owns value, old location can't access
        //don't generally want -> object has been moved to new location
    //mutable reference is temporary exclusive access to value you don't own
        //useful for method to mutate self -> overwrite, etc.
        //need to leave in valid state -> can't move out without replacement
    //shared reference is temporary shared access to non-owned value
        //don't mutate -> good when just observing self



//associate actual cod ewith type using impl
//normal functions inside 'impl' are static
impl List {
    //Self is alias for type next to 'impl'
    //namespacing operator is :: -> use to refer to enum variants
    pub fn push(&mut self, elem: i32){
        //last expression of function implicitly returned
        let new_node = Box::new(Node {
            elem: elem,
            //next: self.head, -> can't move out of borrow context
            next: mem::replace(&mut self.head, Link::Empty),
        });

        //need to return proper self.head -> &mut requirements
        self.head = Link::More(new_node);
        
        //block returns empty tuple '()' since not return value
    }

    //Option<T> is enum representing value that may exist
        //either Some<T> or None -> Option::None and Option::Some imported
    //could make own enum, but Option is ubiquitous and auto-impoted
    //chevrons indicate Option generic over T -> make Option of any type
    pub fn pop(&mut self) -> Option<i32>{
        //use pattern matching to see what Link it is
        //need head of the list by value -> not shared reference
            //since we have mutable reference to self, need to replace
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem) 
            }
        }
        //unimplemented!() is macro (!) -> controlled crash
            //diverging function -> never return to caller
    }

}
