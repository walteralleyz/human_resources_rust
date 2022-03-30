use crate::domain::evento::Evento;
use crate::domain::funcionario::Funcionario;
use crate::utils::date::Date;

pub struct EventoFuncionario<'r> {
    funcionario: &'r Funcionario,
    evento: Evento,
    data_evento: Date,
}
