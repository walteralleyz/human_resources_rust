use rocket::serde::{ Serialize, Deserialize };

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Cargo {
    ASSISTENTE,
    TRAINEE,
    DESENVOLVEDOR,
    GERENTE,
    RECRUTADOR,
}

impl Cargo {
    const CARGOS: [Cargo; 5] = [
        Cargo::ASSISTENTE,
        Cargo::TRAINEE,
        Cargo::DESENVOLVEDOR,
        Cargo::GERENTE,
        Cargo::RECRUTADOR,
    ];

    pub fn from(cargo: &Cargo) -> Cargo {
        match cargo {
            Cargo::ASSISTENTE => Cargo::ASSISTENTE,
            Cargo::TRAINEE => Cargo::TRAINEE,
            Cargo::DESENVOLVEDOR => Cargo::DESENVOLVEDOR,
            Cargo::GERENTE => Cargo::GERENTE,
            Cargo::RECRUTADOR => Cargo::RECRUTADOR,
        }
    }

    pub fn promotable_roles(&self) -> Vec<Cargo> {
        match self {
            Cargo::ASSISTENTE => Vec::from(&Cargo::CARGOS[1..]),
            Cargo::TRAINEE => Vec::from(&Cargo::CARGOS[2..]),
            Cargo::DESENVOLVEDOR | Cargo::RECRUTADOR | Cargo::GERENTE => Vec::from(&Cargo::CARGOS[3..]),
        }
    }

    pub fn to_index(&self) -> i8 {
        for (index, cargo) in Cargo::CARGOS.iter().enumerate() {
            if self == cargo {
                return index as i8;
            }
        }

        -1
    }

    pub fn from_index(index: i32) -> Cargo {
        match Cargo::CARGOS.get(index as usize) {
            Some(cargo) => Cargo::from(cargo),
            None => panic!("Index out of bounds")
        }
    }

    pub fn promote_to(&self, cargo: Cargo) -> Cargo {
        if self.promotable_roles().contains(&cargo) {
            return Cargo::from(&cargo);
        }

        Cargo::from(&self)
    }
}
