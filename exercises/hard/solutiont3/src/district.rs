use std::collections::{HashMap, HashSet};
use std::fs;

pub fn count_provinces() -> String {
    // 读取并解析 JSON 文件
    let json_str = fs::read_to_string("district.json").unwrap();
    let data: HashMap<String, HashMap<String, Vec<String>>> = 
        serde_json::from_str(&json_str).unwrap();
    
    // 存储每个批次的省份数量
    let mut result = Vec::new();
    
    // 处理每个批次
    for i in 1..=5 {
        let batch = data.get(&i.to_string()).unwrap();
        result.push(count_batch_provinces(batch));
    }
    
    // 返回结果字符串
    result.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// 计算单个批次的省份数量
fn count_batch_provinces(batch: &HashMap<String, Vec<String>>) -> u32 {
    let mut union_find = UnionFind::new();
    
    // 将所有城市加入并查集
    for (city, neighbors) in batch {
        union_find.add(city);
        for neighbor in neighbors {
            union_find.add(neighbor);
            union_find.union(city, neighbor);
        }
    }
    
    // 计算不同省份的数量
    union_find.count_provinces()
}

// 并查集实现
struct UnionFind {
    parent: HashMap<String, String>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
        }
    }
    
    fn add(&mut self, city: &str) {
        self.parent.entry(city.to_string())
            .or_insert_with(|| city.to_string());
    }
    
    fn find(&mut self, city: &str) -> String {
        let parent = self.parent.get(city).unwrap().clone();
        if parent == city {
            return city.to_string();
        }
        let root = self.find(&parent);
        self.parent.insert(city.to_string(), root.clone());
        root
    }
    
    fn union(&mut self, city1: &str, city2: &str) {
        let root1 = self.find(city1);
        let root2 = self.find(city2);
        if root1 != root2 {
            self.parent.insert(root2, root1);
        }
    }
    
    fn count_provinces(&mut self) -> u32 {
        let mut provinces = HashSet::new();
        for city in self.parent.keys() {
            provinces.insert(self.find(city));
        }
        provinces.len() as u32
    }
}
