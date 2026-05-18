use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Rc: 複数箇所から同じノードを所有できる
// RefCall: 実行時に借用チェックしながら中身を書き換えられる
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T> {
    key: T,
    next: Link<T>,
    prev: WeakLink<T>,
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    // モジュールで中身のフィールドはカプセル化する場合は、getter が必要。
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_back(&mut self, key: T) {
        let new_tail = Rc::new(RefCell::new(Node {
            key,
            next: None,
            prev: None,
        }));

        // ここの取り出しで、tail を None にしているため、
        // old_tail を所有権付きで操作できる。
        match self.tail.take() {
            Some(old_tail) => {
                new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
        }

        self.len += 1;
    }

    pub fn push_front(&mut self, key: T) {
        let new_head = Rc::new(RefCell::new(Node {
            key,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                new_head.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(Rc::clone(&new_head));
                self.tail = Some(new_head);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // ここで head を None にして、取り出している。
        // map にしているのは、head が None の場合に None を返してくれるのと、
        // map のなかの戻り値を Some で包んで返してくれるから。
        self.head.take().map(|old_head| {
            let next = old_head.borrow_mut().next.take();

            match next {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }

            self.len -= 1;

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().key
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            let prev = old_tail
                .borrow_mut()
                .prev
                .take()
                // その後に値を mutation するために、共有所有権を取得
                // upgrade() が None が変える場合は参照が free されている。
                .and_then(|weak| weak.upgrade());

            match prev {
                Some(new_tail) => {
                    // Rc に変換しているため、ここで mutation できる。
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }

            self.len -= 1;

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().key
        })
    }

    // 合致する key を持つ先頭の Node を削除
    pub fn delete(&mut self, key: T) -> Option<T>
    where
        T: PartialEq,
    {
        let mut current = self.head.clone();

        while let Some(node) = current {
            let key_matches = node.borrow().key == key;

            if key_matches {
                if self
                    .head
                    .as_ref()
                    .is_some_and(|head| Rc::ptr_eq(head, &node))
                {
                    drop(node);
                    return self.pop_front();
                }

                if self
                    .tail
                    .as_ref()
                    .is_some_and(|tail| Rc::ptr_eq(tail, &node))
                {
                    drop(node);
                    return self.pop_back();
                }

                let (prev, next) = {
                    let mut node_ref = node.borrow_mut();
                    let prev = node_ref.prev.take().and_then(|weak| weak.upgrade());
                    let next = node_ref.next.take();
                    (prev, next)
                };

                if let (Some(prev), Some(next)) = (prev, next) {
                    prev.borrow_mut().next = Some(Rc::clone(&next));
                    next.borrow_mut().prev = Some(Rc::downgrade(&prev));
                }

                self.len -= 1;

                return Some(Rc::try_unwrap(node).ok().unwrap().into_inner().key);
            }

            let next = node.borrow().next.clone();
            current = next;
        }

        None
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut keys = Vec::with_capacity(self.len);
        let mut current = self.head.clone();

        while let Some(node) = current {
            let node_ref = node.borrow();
            keys.push(node_ref.key.clone());
            current = node_ref.next.clone();
        }

        keys
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        DoublyLinkedList::new()
    }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;

    #[test]
    fn delete_head() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.delete(1), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn delete_tail() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.delete(2), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn delete_middle() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.delete(2), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(3));
        assert!(list.is_empty());
    }

    #[test]
    fn delete_missing_value() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);

        assert_eq!(list.delete(2), None);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn to_vec_returns_keys_from_head_to_tail() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);

        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }
}
