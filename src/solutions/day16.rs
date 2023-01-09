use std::collections::HashMap;


pub fn part1(input: &str) -> u32 {
    0
}

pub fn part2(input: &str) -> u32 {
    0
}

struct Graphe {
    vertexes: HashMap<u8, (u32, Vec<u8>)>
}

impl Graphe {
    fn reduce(&mut self) {
        let Some((idx, destination)) = self.vertexes.iter()
            .find_map(|(idx, (rate, destination))| if *rate == 0 { Some((*idx, destination.clone())) } else {None}) else {
                return;
            };
        
        self.vertexes.iter_mut()
            .filter_map(|(_, (_, go_to))| if let Some(idx) = go_to.iter().position(|go_to_idx| *go_to_idx == idx) {
                Some((idx, go_to))
            } else {
                None
            }
            
        )            
            .for_each(|(vertex_idx, go_to)| {
                go_to.remove(vertex_idx);
                go_to.append(&mut destination.clone())
            });
    }
}