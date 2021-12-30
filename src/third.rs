use std::mem;

/*
move from single ownership to shared ownership
make persistent immutable singly-linked list

same list used in functional programming
    can only get head, tail, or concatenate

key feature: persistent lists have efficeint tail manipulation
    pointers should be shared to save memory
    this won't work with shared Boxes -> will free

Normally done with garbage collection when everyone done
    Tracing GC: digs through memory at runtime and identify garbage
    Rust has reference counting: very simple GC
        less throughput and fails if 'build cycles'

Reference-counted GC -> use 'Rc', like Box but can duplicate it
    Only free memory when ALL Rc's desired from it are dropped
    Can only take shared reference to internals
        Can't get data out or mutate
*/

use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    //replace push with prepend for immutable list
    //take list and element, return List
        //Clone trait implemented by almost every type
        //Logically disjoint given only shared reference
        //Never implicitly invoked -> used by Rc
    
    //don't need to mathc on head -> Option exposes Clone implementation
    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    //replace pop with tail -> return whole list with first element removed
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    //head provides reference to first element -> just peek from mutable list
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }
    
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

//Iter is identical to structure of mutable list
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


//Can't implement IntoIter or IterMut -> only have shared access to elements

//Have recursive descturo problem again -> can't mutate Node inside the Box
    //Rc only gives shared access
//But if we know we're the last list that knows about thise node
    //It would be find to move Node out of Rc
    //Then know when to stop, whenever we can't hoist ou the Node
    
    //Rc has 'try_unwrap' method for this

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

/*
Immutable linked list useful for shared data across threads
Thread-safe requires atomic reference counces
    Otherwise two threads could increment reference count

Use Arc for thread safety
    Same as Rc but with atomicity for reference counts -> overhead

Simply replace every Rc reference with Arc
    'Rc' -> 'std::sync::Arc'

In generally, can't mess up thread-safety in Rust
    Thread-safety modeled with Send and Sync traits

Type is Send if safe to move to another thread
Type is Sync is safe to share between multiple threads
    Safe meaning avoids data races
So if T is Sync then &T is Send

Thse are marker traits: specifically defined property, no interface
    If value isn't Send, then statically impossible to be sent

Almost every type is Send and Sync
    Most types are Send -> own their own data
    Most types are Sync -> share data across threads by putting behind shared reference (immutable)

Special types violate properties -> have interior mutability
    Can mutate through shared reference -> two types
        Cells: only in single-threaded context
        Locks: work in multi-threaded context
            Atomics: primitives that act like lock
Contrast with inherited mutability (external mutability)
    Mutability of value is inherited from mutability of its container
    Can't just randomly mutate some field of non-mutable value

Rc and Arc both use interior mutability for reference count
    Reference count is shared between every instance
    Rc uses cell -> not thread-safe but lessoverhead
    Arc uses atomic -> thread-safe

*/

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}