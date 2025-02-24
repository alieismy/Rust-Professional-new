use std::collections::{HashMap, HashSet};
use std::fs;
use serde_json::{Value, Deserializer};

// 并查集结构
struct UnionFind {
    parent: HashMap<String, String>,
    rank: HashMap<String, usize>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn find(&mut self, city: &str) -> String {
        let p = self.parent.entry(city.to_string()).or_insert_with(|| {
            self.rank.insert(city.to_string(), 1);
            city.to_string()
        }).clone();
        
        if p != city {
            let root = self.find(&p);
            self.parent.insert(city.to_string(), root.clone());
        }
        self.parent[city].clone()
    }

    fn union(&mut self, x: &str, y: &str) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return;
        }

        let rank_x = self.rank[&root_x];
        let rank_y = self.rank[&root_y];
        
        if rank_x > rank_y {
            self.parent.insert(root_y.clone(), root_x.clone());
        } else {
            self.parent.insert(root_x.clone(), root_y.clone());
            if rank_x == rank_y {
                *self.rank.get_mut(&root_y).unwrap() += 1;
            }
        }
    }

    fn count_roots(&self) -> i32 {
        let mut roots = HashSet::new();
        for (city, parent) in &self.parent {
            if city == parent {
                roots.insert(parent);
            }
        }
        roots.len() as i32
    }
}

pub fn count_provinces() -> String {
    // 流式解析JSON
    let file = fs::File::open("district.json").unwrap();
    let stream = Deserializer::from_reader(file).into_iter::<Value>();
    
    let mut results = Vec::new();
    
    for value in stream {
        if let Ok(json) = value {
            if let Some(batches) = json.as_object() {
                // 按批次顺序处理1-5
                for batch_num in 1..=5 {
                    let mut uf = UnionFind::new();
                    if let Some(batch) = batches.get(&batch_num.to_string()) {
                        if let Some(cities) = batch.as_object() {
                            // 收集所有城市节点（包括邻居中提到的）
                            let mut all_cities = HashSet::new();
                            for (city, neighbors) in cities {
                                all_cities.insert(city.as_str());
                                if let Some(neighbors_array) = neighbors.as_array() {
                                    for neighbor in neighbors_array {
                                        if let Some(neighbor_str) = neighbor.as_str() {
                                            all_cities.insert(neighbor_str);
                                        }
                                    }
                                }
                            }

                            // 初始化所有节点
                            for &city in &all_cities {
                                uf.find(city);
                            }

                            // 建立连接关系
                            for (city, neighbors) in cities {
                                if let Some(neighbors_array) = neighbors.as_array() {
                                    for neighbor in neighbors_array {
                                        if let Some(neighbor_str) = neighbor.as_str() {
                                            if city != neighbor_str && all_cities.contains(neighbor_str) {
                                                uf.union(city.as_str(), neighbor_str);
                                            }
                                        }
                                    }
                                }
                            }
                                        }
                                    }
                                }
                            }

                            let count = uf.count_roots();
                            results.push(count);
                        }
                    } else {
                        results.push(0); // 处理缺失批次
                    }
                }
            }
        }
    }
    
    // 转换为逗号分隔的字符串
    results.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
                            // 第一遍处理所有节点
                            for city in cities.keys() {
                                uf.find(city);
                            }
                            // 第二遍处理边关系
                            // 使用有序处理确保所有节点都被注册
                            let mut all_cities = HashSet::new();
                            for (city, neighbors) in cities {
                                all_cities.insert(city.as_str());
                                if let Some(neighbors_array) = neighbors.as_array() {
                                    for neighbor in neighbors_array {
                                        if let Some(neighbor_str) = neighbor.as_str() {
                                            all_cities.insert(neighbor_str);
                                        }
                                    }
                                }
                            }
                            
                            // 初始化所有节点
                            for &city in &all_cities {
                                uf.find(city);
                            }
                            
                            // 建立连接关系
                            for (city, neighbors) in cities {
                                if let Some(neighbors_array) = neighbors.as_array() {
                                    for neighbor in neighbors_array {
                                        if let Some(neighbor_str) = neighbor.as_str() {
                                            if city != neighbor_str {
                                                uf.union(city, neighbor_str);
                                            }
                                        }
                                    }
                                }
                            }
                            let count = uf.count_roots();
                            results.push(count.to_string());
                            continue;
                        }
                    }
                    results.push("0".to_string()); // 处理缺失批次
                }
            }
        }
    }
    
    results.join(",")
}
