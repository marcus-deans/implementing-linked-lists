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
// No lifetime here, List doesn't have any associated lifetimes
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

    pub fn peek(&self) -> Option<&T> {
        //can't directly return reference to element in head of list
            //map takes 'self' by value -> move Option out of thing it was in
            //peek is leaving in place -> use as_ref method to handle it
            //as_ref implemented with
                //impl<T> Option<T> {
                    // pub fn as_ref(&self) -> Option<&T>;
                // }
            //Option is reference to internals -> deference with '.'
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    //mutable version of method using as_mut
    pub fn peek_mut(&mut self) -> Option<&mut T>{
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /* We declare a fresh lifetime here for the *exact* borrow that
    creates the iter. Now &self needs to be valid as long as the
    Iter is around.

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_derefer() }
    }

     Lifetime elision actually applies here
    Show that struct contains lifetime using 'explicitly elided lifetime'
    Uses '_
    */
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
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

/*
iterate collections using Iterator trait
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    every Iterator implement has associated type 'Item'
    Iterator yields Option<Self::Item> since it coaleseces has_next and get_next
        when have next_value, yield Some(value)
        when don't have next_value, yield None

    Rust doesn't have yield statement -> so implement manually
    Each collection should try and implement 3 iterators:
        IntoIter - T
        IterMut - &mut T
        Iter - &T    
*/

//implement IntoIter with repeated pops
//Tuple struct is alternative form of struct
    //useful for trivial wrappers around other types
pub struct IntoIter<T>(List<T>);

//also include below method within List implementation
/*
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
*/

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        //access fields of a tuple struct numerically
        self.0.pop()
    }
}


/*
Most Rust data manually managed -> no garbage collector to control lifetimes
    Need to prevent having pointers to random data on stack (pervasive unmanageable unsafety)
        Holding pointer to something that went out of scope
        Holding pointer to something that got mutated away
    Lifetimes solve these problems transparently
        Lifetime is name of region(~block/scope) of code in program
        Tag reference with lifetime -> lifetime valid for entire region

        Compiler works if it finds set of lifetimes that satisfy all constraints
        Wqithin function body, compiler knows information and constraints -> determines lifetime
        Compiler needs information at type and API-level 

        Providing this information allows borrow checking done in each function independently
        Erors will be mostly local, or types have incorect signatures

        Previously wrote references in function signatures
        Rust uses 'lifetime elision' in common cases to auto-picki lifetimes

        // Only one reference in input, so the output must be derived from that input
        fn foo(&A) -> &B; // sugar for:
        fn foo<'a>(&'a A) -> &'a B;

        // Many inputs, assume they're all independent
        fn foo(&A, &B, &C); // sugar for:
        fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

        // Methods, assume all output lifetimes are derived from `self`
        fn foo(&self, &B, &C) -> &D; // sugar for:
        fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;

        then fn foo<'a>(&'a A) -> &'a B 
            means input must live at least as long as output
            if output kept around, then input must be valid for larger region
        allows Rust to ensure nothing is used after free
            Guarantees that constraints work out
            Nothing mutated while outstanding references exist
*/


//Iter should hold pointer to current node that we want to yield next
    //node may not exist (empty or done iterator) -> use Option
    //when yield element, proceed to current node's 'next' node
// Iter is generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

//again implement functionality within List implementation

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

/*
as_deref replaces map(|node| &**node)
as_deref_mut replaces map(|node| &mut**node)
Normally don't need to use -> Rust converts implicitly 
    Uses 'deref coercion' -> inserting *'s inside to type-check
    Uses borrow checker to avoid messing up pointers
Complexity of closure with Option<&T> instead of &T requires it

Could give it a hinrt with 'turbofish'
    self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);

    map is generic function: pub fn map<U, F>(self, f: F) -> Option<U>

    turbofish is ::<> lets us tell compiler what generic types should be
        ::<&Node<T>, _> says it should return &Node<T> and something unknown
    
    Then compiler knows that &node should have deref coercion applied
*/

#[cfg(test)] //indicates to only compile 'test' when running tests
mod test {
    //made new module -> need to pull List explicitly to use it
    use super::List;
    
    #[test] //testing annotation
    fn basic(){
        //assert_eq! macro
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        //CAN'T DO
            // list.peek_mut().map(|&mut value| {
            //     value = 42
            // });
        // |&mut value| = argument is mutable reference, but just copy value
        // |value| = type of value is &mut i32 -> can mutate head

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}