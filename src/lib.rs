pub type ProgramId = usize;

struct Program {
    id: ProgramId,
    net: Vec<ProgramId>,
}

impl Program {
    fn nets(&self) -> &[usize] {
        &self.net
    }
}

impl std::str::FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitter = s.splitn(2, " <-> ");
        let id_str = splitter.next().ok_or(format!("Invalid program line '{}'", s))?;
        let net_str = splitter.next().ok_or(format!("Invalid program line '{}'", s))?;
        Ok(Program {
            id: id_str.parse().map_err(|e| format!("{}", e))?,
            net: net_str.split(',').map(|el| el.trim().parse().unwrap()).collect(),
        }
        )
    }
}

pub struct Configurations {
    programs: Vec<Program>
}

impl std::str::FromStr for Configurations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Configurations {
                programs: s.lines().map(|l| l.parse().unwrap()).collect(),
            }
        )
    }
}

#[derive(Clone)]
pub struct Set {
    pub id: ProgramId,
    pub size: usize,
    parent: ProgramId,
}

impl Set {
    fn new(id: ProgramId) -> Self {
        Set {
            id,
            size: 1,
            parent: id,
        }
    }
}

impl Configurations {}

use std::collections::HashMap;

pub struct Sets {
    pub map: HashMap<ProgramId, Set>
}

impl Sets {
    pub fn get(&self, id: ProgramId) -> Set {
        self.map[&self.root(id)].clone()
    }

    pub fn root(&self, mut id: ProgramId) -> ProgramId {
        while id != self.map[&id].parent {
            id = self.map[&id].parent;
        }
        id
    }

    fn join(&mut self, a: ProgramId, b: ProgramId) {
        let mut parent;
        let mut child;

        {
            let a = &self.map[&self.root(a)];
            let b = &self.map[&self.root(b)];

            parent = a.id;
            child = b.id;
            if a.size < b.size {
                parent = b.id;
                child = a.id;
            }
        }
        if child != parent {
            self.join_roots(child, parent);
        }
    }

    fn join_roots(&mut self, child: ProgramId, parent: ProgramId) {
        self.map.get_mut(&child).unwrap().parent = parent;
        self.map.get_mut(&parent).unwrap().size += self.map[&child].size
    }
}

impl From<Configurations> for Sets {
    fn from(conf: Configurations) -> Self {
        let mut result = Self {
            map: conf.programs.iter().map(|p| (p.id, Set::new(p.id))).collect()
        };
        conf.programs.iter().map(|p| (p, p.id)).
            flat_map(|(p, pid)|
            p.nets().iter().map(move|&d| (pid, d)))
            .map(|(a,b)| result.join(a,b)).count();
        result
    }
}

pub fn get_sets(conf: Configurations) -> Sets {
    conf.into()
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &'static str = "\
                                0 <-> 2\n\
                                1 <-> 1\n\
                                2 <-> 0, 3, 4\n\
                                3 <-> 2, 4\n\
                                4 <-> 2, 3, 6\n\
                                5 <-> 6\n\
                                6 <-> 4, 5\
                                ";

    #[test]
    fn read_data() {
        let configurations = DATA.parse::<Configurations>().unwrap();

        assert_eq!(7, configurations.programs.len());
        assert_eq!(3, configurations.programs[3].id);
        assert_eq!(&[0, 3, 4], configurations.programs[2].nets());
    }

    #[test]
    fn one_set() {
        let configurations = "\
        0 <-> 0\
        ".parse::<Configurations>().unwrap();

        let sets = get_sets(configurations);

        assert_eq!(1, sets.get(0).size);
    }

    #[test]
    fn two_joined_sets() {
        let configurations = "\
        0 <-> 1\n\
        1 <-> 0\
        ".parse::<Configurations>().unwrap();

        let sets = get_sets(configurations);

        assert_eq!(2, sets.get(0).size);
        assert_eq!(2, sets.get(1).size);
    }

    #[test]
    fn integration() {
        let sets: Sets = DATA.parse::<Configurations>().unwrap().into();

        assert_eq!(6, sets.get(0).size);
        assert_eq!(1, sets.get(1).size);
    }
}
