use std::collections::HashSet;
use std::cmp::Ordering;

struct Tubes {
    tubes: HashSet<(usize, usize)>,
}

impl Tubes {
    fn parse_tube(s: &str) -> Option<(usize, usize)> {
        let v: Vec<usize> = s.trim()
            .split('/')
            .filter_map(|x| x.parse().map_err(|e| eprintln!("{}", e)).ok())
            .collect();
        if v.len() == 2 {
            Some((v[0], v[1]))
        } else {
            None
        }
    }

    fn parse_tubes(s: &str) -> Self {
        let tubes = s.trim().split('\n').filter_map(Self::parse_tube).collect();
        Tubes { tubes }
    }

    fn compatible(&self, port: usize) -> Vec<(usize, usize)> {
        self.tubes
            .iter()
            .filter(|&&(a, b)| port == a || port == b)
            .cloned()
            .collect()
    }

    fn extend_strong(&mut self, port: usize) -> Vec<(usize, usize)> {
        self.compatible(port)
            .into_iter()
            .map(|tube| {
                let other = other_port(&tube, port);
                self.tubes.remove(&tube);
                let mut bridge = vec![tube];
                bridge.append(&mut self.extend_strong(other));
                self.tubes.insert(tube);
                bridge
            })
            .max_by_key(|bridge| strength(bridge))
            .unwrap_or_else(|| vec![])
    }

    fn extend_long(&mut self, port: usize) -> Vec<(usize, usize)> {
        self.compatible(port)
            .into_iter()
            .map(|tube| {
                let other = other_port(&tube, port);
                self.tubes.remove(&tube);
                let mut bridge = vec![tube];
                bridge.append(&mut self.extend_long(other));
                self.tubes.insert(tube);
                bridge
            })
            .max_by(|a, b| cmp_bridge(a, b))
            .unwrap_or_else(|| vec![])
    }
}

fn other_port(tube: &(usize, usize), port: usize) -> usize {
    let &(a, b) = tube;
    if port == a {
        b
    } else {
        a
    }
}

fn strength(bridge: &[(usize, usize)]) -> usize {
    bridge.into_iter().map(|&(a, b)| a + b).sum()
}

pub fn one(s: &str) -> String {
    let mut tubes = Tubes::parse_tubes(s);
    let bridge = tubes.extend_strong(0);
    strength(&bridge).to_string()
}

fn cmp_bridge(a: &[(usize, usize)], b: &[(usize, usize)]) -> Ordering {
    a.len().cmp(&b.len()).then(strength(a).cmp(&strength(b)))
}

pub fn two(s: &str) -> String {
    let mut tubes = Tubes::parse_tubes(s);
    let bridge = tubes.extend_long(0);
    strength(&bridge).to_string()
}
