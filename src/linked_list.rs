use std::ptr;

pub(crate) struct LinkedListIterator<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let data = &self.node?.data;
        self.node = self.node?.next();
        Some(data)
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = LinkedListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator{node: self.head()}
    }
}

pub(crate) struct Node<T> {
    pub data: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { data, next: ptr::null_mut(), prev: ptr::null_mut() }
    }

    pub fn own(&self) -> Node<T> {
        unsafe {
            (self as *const Node<T>).read()
        }
    }

    pub fn new_ptr(data: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Self::new(data)))
    }

    pub fn next(&self) -> Option<&Node<T>> {
        unsafe {
            self.next.as_ref()
        }
    }

    pub fn next_mut(&self) -> Option<&mut Node<T>> {
        unsafe {
            self.prev.as_mut()
        }
    }

    pub fn prev(&self) -> Option<&Node<T>> {
        unsafe {
            self.prev.as_ref()
        }
    }

    pub fn prev_mut(&self) -> Option<&mut Node<T>> {
        unsafe {
            self.prev.as_mut()
        }
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}

pub(crate) struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {head: ptr::null_mut(), tail: ptr::null_mut(), len: 0}
    }

    pub fn push(&mut self, data: T) {
        self.len += 1;
        let node = Node::new_ptr(data);
        if self.head.is_null() {
            self.head = node;
            self.tail = node;
            return;
        }
        unsafe {
            (*self.tail).next = node;
            (*node).prev = self.tail;
            self.tail = node;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn head_mut(&self) -> Option<&mut Node<T>> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            Some(&mut *self.head)
        }
    }

    pub fn head(&self) -> Option<&Node<T>> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            Some(&*self.head)
        }
    }

    pub fn tail_mut(&self) -> Option<&mut Node<T>> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            Some(&mut *self.tail)
        }
    }

    pub fn tail(&self) -> Option<&Node<T>> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            Some(&*self.tail)
        }
    }

    pub fn remove(&mut self, node: &Node<T>) -> Option<T> {
        if node.prev.is_null() {
            self.remove_first()
        } else {
            self.remove_next(node.prev_mut().unwrap())
        }
    }

    pub fn remove_first(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        self.len -= 1;
        unsafe {
            let old = self.head;
            self.head = (*self.head).next;
            if old == self.tail {
                self.tail = self.head;
            }
            let data = Box::from_raw(old).data;
            (*self.head).prev = std::ptr::null_mut();
            Some(data)
        }
    }

    pub fn remove_last(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        }
        self.len -= 1;
        unsafe {
            let old = self.tail;
            self.tail = (*self.tail).prev;
            if old == self.head {
                self.head = self.tail;
            }
            let data = Box::from_raw(old).data;
            (*self.tail).next = std::ptr::null_mut();
            Some(data)
        }
    }

    pub fn remove_next(&mut self, node: &Node<T>) -> Option<T> {
        if node.next.is_null() {
            return None;
        }
        if node.next == self.tail {
            return self.remove_last();
        }
        self.len -= 1;
        unsafe {
            let old = node.next;
            node.own().next = (*old).next;
            (*node.next).prev = &mut node.own() as *mut Node<T>;
            let data = Box::from_raw(old).data;
            Some(data)
        }
    }

    pub fn remove_prev(&mut self, node: &Node<T>) -> Option<T> {
        if node.prev.is_null() {
            return None;
        }
        if node.prev == self.head {
            return self.remove_first();
        }
        self.len -= 1;
        unsafe {
            let old = node.prev;
            node.own().prev = (*old).prev;
            (*node.prev).next = &mut node.own() as *mut Node<T>;
            let data = Box::from_raw(old).data;
            Some(data)
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut node = self.head;
        while !node.is_null() {
            unsafe {
                let old = node;
                let next = node.read().next;
                (*node).next = ptr::null_mut();
                node = next;
                drop(Box::from_raw(old));
            }
        }
    }
}