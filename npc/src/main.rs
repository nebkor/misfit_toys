use std::collections::HashMap;

#[derive(Clone)]
enum Class {
    Elf,
    Dwarf,
}

#[derive(Clone)]
enum Traits {
    Class(Class),
    Size(f64),
    Name(String),
}

#[derive(Clone)]
enum LockableTrait {
    Locked(Traits),
    Unlocked(Traits),
}

impl LockableTrait {
    pub fn unwrap(&self) -> Option<Traits> {}
}

struct Npc {
    traits: HashMap<String, LockableTrait>,

    traits_map: HashMap<String, Traits>,
}

impl Default for Npc {
    fn default() -> Self {
        let mut traits_map = HashMap::new();
        traits_map.insert("name".to_string(), Traits::Name(String));
        Npc {
            traits: HashMap::default(),
            traits_map,
        }
    }
}

impl Npc {
    pub fn add_locked(&mut self, locked_trait: Traits) {
        self.traits.insert(
            traits_map.get(&self.locked_trait),
            LockableTrait::Locked(locked_trait),
        );
    }

    pub fn add_unlocked(&mut self, unlocked_trait: Traits) {
        self.traits.push(LockableTrait::Unlocked(unlocked_trait));
    }

    pub fn name(&self) -> Option<String> {
        npc.traits
            .get(&self.traits_map.get("name").unwrap())
            .map(|name| {
                let Some(Name(name)) = name.unwrap();
                Some(name)
            })
    }
}

fn main() {
    let mut npc = Npc::default();

    npc.traits_map.insert("name", Name("namey name-face"));

    npc.add_locked(Traits::Class(Class::Dwarf));
    npc.add_unlocked(Traits::Size(1.0));
}
