pub enum Evento {
    CONTRATACAO,
    DEMISSAO,
    PROMOCAO
}

impl Evento {
    pub fn to_index(&self) -> u8 {
        match self {
            Evento::CONTRATACAO => 1,
            Evento::DEMISSAO => 2,
            Evento::PROMOCAO => 3
        }
    }
}
