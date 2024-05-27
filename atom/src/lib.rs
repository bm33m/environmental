/*
DIY project no. 25001
Write a trait for an atom model.

DIY project no. 25004
Write a rust crate to implement a trait for an atom model, using the electronics semiconductor theory  and the periodic table as reference point.


*/

pub trait Atom {
    fn get_code_name(&self) -> (bool, String);
    fn description(&self) -> String;
    fn get_electrons(&self) -> i32;
    fn get_protons(&self) -> i32;
    fn get_neutrons(&self) -> f32;
    fn get_nucleus(&self) -> Nucleus;
}

pub struct Nucleus {
    pub protons: i32,
    pub neutrons: f32,
}

#[derive(Clone, Debug)]
pub struct AtomModel {
    pub code: String,
    pub name: String,
    pub atomic_number: i32,
    pub atomic_mass: f64,
    pub valence_electrons: i32,
    pub classification: String,
    material: MaterialType,
}

impl Atom for AtomModel {
    fn get_code_name(&self) -> (bool, String) {
        let code = &self.code;
        let name = &self.name;
        let len = (*code).len();
        assert!(len <= (*name).len());
        let mut code_name: String = String::new();
        let mut i = 0;
        for x in name.chars() {
            code_name.push(x);
            i += 1;
            if i == len {
                break;
            }
        }
        ((code_name == *code), code_name)
    }

    fn description(&self) -> String {
        format!("name: {}, code: {}, classification: {}", self.name, self.code,
            self.classification)
    }

    fn get_electrons(&self) -> i32 {
        self.atomic_number
    }

    fn get_protons(&self) -> i32 {
        self.atomic_number
    }

    fn get_neutrons(&self) -> f32 {
        let neutrons: f64 = self.atomic_mass - (self.atomic_number as f64);
        neutrons as f32
    }

    fn get_nucleus(&self) -> Nucleus {
        Nucleus {
            protons: self.get_protons(),
            neutrons: self.get_neutrons(),
        }
    }

}

#[derive(Clone, Debug)]
enum Semiconductor {
    Silicon,
    Germanium,
}

#[derive(Clone, Debug)]
enum MaterialType {
    IntrinsicSemiconductor(Semiconductor),
    ExtrinsicSemiconductor(Semiconductor),
    Conductor(String),
    NonConductor(String),
    ExtrinsicMaterial(String),
    Other(String),
}

const TETRAVALENT: i32 = 4;
const BROKENBOND: i32 = 3;
const PENTRAVALENT: i32 = 5;

impl AtomModel {
    pub fn doping(&mut self, substance: &mut AtomModel) -> bool {
        assert!(self.valence_electrons == TETRAVALENT);
        let mut results = false;
        let silicon = "Silicon".to_string();
        let germanium = "Germanium".to_string();
        let material = self.material.clone();
        match material {
            MaterialType::IntrinsicSemiconductor(Semiconductor::Silicon) => {
                print_atom(silicon);
                results = self.dop(substance, &mut results);
            },
            MaterialType::IntrinsicSemiconductor(Semiconductor::Germanium) => {
                print_atom(germanium);
                results = self.dop(substance, &mut results);
            },
            MaterialType::Conductor(_) | MaterialType::NonConductor(_) => todo!(),
            MaterialType::ExtrinsicSemiconductor(_) | MaterialType::ExtrinsicMaterial(_) => todo!(),
            MaterialType::Other(_) => todo!(),
        }
        results
    }

    fn dop(&mut self, substance: &mut AtomModel, results: &mut bool)-> bool {
        let semiconductors = ["Silicon".to_string(), "Germanium".to_string(),];
        assert!(semiconductors.contains(&self.name));
        let ptype_material_substance = ["Boron".to_string(), "Gallium".to_string(), "Indium".to_string(),];
        let ntype_material_substance = ["Antinomy".to_string(), "Arsenic".to_string(), "Phosphorus".to_string(),];
        let valence_electrons_x = substance.valence_electrons;
        if valence_electrons_x == BROKENBOND {
            assert!(ptype_material_substance.contains(&substance.name));
            self.valence_electrons += BROKENBOND;
            substance.valence_electrons -= BROKENBOND;
            *results = true;
        } else if valence_electrons_x == PENTRAVALENT {
            assert!(ntype_material_substance.contains(&substance.name));
            self.valence_electrons += TETRAVALENT;
            substance.valence_electrons -= TETRAVALENT;
            *results = true;
        }
        let name = self.name.clone();
        self.material = get_material("ExtrinsicSemiconductor", name);
        *results
    }

    pub fn other(&mut self) {
        let name = self.name.clone();
        self.material = MaterialType::Other(name);
    }

    pub fn conductor(&mut self) {
        let name = self.name.clone();
        self.material = MaterialType::Conductor(name);
    }

    pub fn non_conductor(&mut self) {
        let name = self.name.clone();
        self.material = MaterialType::NonConductor(name);
    }

    pub fn extrinsic_material(&mut self, name: String) {
        let name_x = name.clone();
        self.material = MaterialType::ExtrinsicMaterial(name_x);
    }

    pub fn intrinsic_semiconductor(&mut self, name: String) {
        self.material = get_material("Semiconductor", name);
    }

}

pub fn new(type_x: &str, name: String, code: String,
    atomic_number: i32, atomic_mass: f64, valence_electrons: i32)-> AtomModel {
    let atom1 = AtomModel {
        code: code,
        name: name.clone(),
        atomic_number: atomic_number,
        atomic_mass: atomic_mass,
        valence_electrons: valence_electrons,
        classification: String::new(),
        material: get_material(type_x, name),
    };
    atom1
}

fn get_material(type_x: &str, name: String)-> MaterialType {
    if type_x == "Semiconductor" {
        if name == ("Silicon".to_string()) {
          return MaterialType::IntrinsicSemiconductor(Semiconductor::Silicon);
        };
        if name == ("Germanium".to_string()) {
          return MaterialType::IntrinsicSemiconductor(Semiconductor::Germanium);
        };
    } else if type_x == "ExtrinsicSemiconductor" {
        if name == ("Silicon".to_string()) {
          return MaterialType::ExtrinsicSemiconductor(Semiconductor::Silicon);
        };
        if name == ("Germanium".to_string()) {
          return MaterialType::ExtrinsicSemiconductor(Semiconductor::Germanium);
        };
    } else if type_x == "Substance" {
       return MaterialType::ExtrinsicMaterial(name);
    };
    return MaterialType::Other(name);
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn print_atom(atom: String) {
    println!("{}", atom);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_atom() {
        let mut atom1 = AtomModel {
            code: "Si".to_string(),
            name: "Silicon".to_string(),
            atomic_number: 14,
            atomic_mass: 28.00,
            valence_electrons: 4,
            classification: String::new(),
            material: get_material("Semiconductor", "Silicon".to_string()),
        };
        let mut atom2 = AtomModel {
            code: "Sb".to_string(),
            name: "Antinomy".to_string(),
            atomic_number: 51,
            atomic_mass: 122.00,
            valence_electrons: 5,
            classification: String::new(),
            material: MaterialType::ExtrinsicMaterial("Antinomy".to_string()),
        };
        let result = atom1.doping(&mut atom2);
        assert!(result == true);
        assert!((atom1.valence_electrons) == 8);
        assert!((atom2.valence_electrons) == 1);
    }
}
