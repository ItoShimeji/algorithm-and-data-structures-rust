pub struct Heap {
    heap: Vec<i64>,
}

// key: i64
// node id (i): usize <- 1 origin
// index: usize <- 0 origin
impl Heap {
    pub fn new() -> Self {
        Heap { heap: Vec::new() }
    }

    pub fn insert(&mut self, key: i64) {
        // key を heap の最終要素に追加し、heap 条件を満たすところまで上昇させる
        self.heap.push(key);
        self.increase_key(self.heap.len());
    }

    fn increase_key(&mut self, id: usize) {
        // None の時は root
        if let Some(parent_id) = parent_id(id) {
            if self.heap[parent_id - 1] < self.heap[id - 1] {
                // 親と自分を交換
                self.heap.swap(id - 1, parent_id - 1);
                // 親の位置に映った自分を再度検証
                self.increase_key(parent_id);
            }
        }
    }

    pub fn extract_max(&mut self) -> Option<i64> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let max = self.heap.pop();

        self.max_heapify(1);

        max
    }

    fn max_heapify(&mut self, id: usize) {
        let left = self.heap.get(left_id(id) - 1).copied();
        let right = self.heap.get(right_id(id) - 1).copied();

        // この時点で、i より下の木構造は max heap になっている前提
        let mut largest = id;
        if let Some(left_value) = left {
            if self.heap[id - 1] < left_value {
                largest = left_id(id);
            }
        }
        if let Some(right_value) = right {
            if self.heap[largest - 1] < right_value {
                largest = right_id(id);
            }
        }

        if largest != id {
            self.heap.swap(id - 1, largest - 1);
            // largest は現在 i が存在している
            self.max_heapify(largest);
        }
    }
}

fn parent_id(id: usize) -> Option<usize> {
    if id == 1 {
        None
    } else {
        Some(id / 2)
    }
}

fn left_id(id: usize) -> usize {
    2 * id
}

fn right_id(id: usize) -> usize {
    2 * id + 1
}
