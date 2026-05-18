struct Entry<T> {
    key: String,
    value: T,
}

pub struct HashMap<T> {
    size: usize,
    salt: usize,
    buckets: Vec<Vec<Entry<T>>>,
}

impl<T> HashMap<T> {
    pub fn new(size: usize, salt: usize) -> HashMap<T> {
        HashMap {
            size,
            salt,
            // buckets は size 分先に chain を埋めておく。
            buckets: (0..size).map(|_| Vec::new()).collect(),
        }
    }

    fn hash(&self, key: &str) -> usize {
        key.bytes().map(|b| b as usize).sum::<usize>() + self.salt
    }

    fn get_chain(&self, key: &str) -> &Vec<Entry<T>> {
        let chain_index = self.hash(key) % self.size;
        // 初期化時に埋めているため、必ず存在する。
        self.buckets.get(chain_index).unwrap()
    }

    // mut 専用のメソッドを定義
    fn get_chain_mut(&mut self, key: &str) -> &mut Vec<Entry<T>> {
        let chain_index = self.hash(key) % self.size;
        // 初期化時に埋めているため、必ず存在する。
        self.buckets.get_mut(chain_index).unwrap()
    }

    pub fn insert(&mut self, key: &str, value: T) {
        let chain = self.get_chain_mut(key);

        // すでに当該の key が存在する場合は無視
        let has_entry = chain.iter().find(|entry| entry.key == key).is_some();
        if !has_entry {
            chain.push(Entry {
                key: key.to_string(),
                value,
            });
        }
    }

    pub fn find(&self, key: &str) -> Option<&T> {
        let chain = self.get_chain(key);

        // もっと良い値の返し方ありそう。
        match chain.iter().find(|entry| entry.key == key) {
            Some(entry) => Some(&entry.value),
            None => None,
        }
    }
}
