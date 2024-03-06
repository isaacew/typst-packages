use serde::{Deserialize, Serialize};

/// Which disciplines this package is useful for.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Discipline {
    Agriculture,
    Anthropology,
    Archaeology,
    Architecture,
    Biology,
    Business,
    Chemistry,
    Communication,
    ComputerScience,
    Design,
    Drawing,
    Economics,
    Education,
    Engineering,
    Environment,
    Fashion,
    Film,
    Geography,
    Geology,
    History,
    Journalism,
    Law,
    Linguistics,
    Literature,
    Mathematics,
    Medicine,
    Music,
    Painting,
    Philosophy,
    Photography,
    Physics,
    Politics,
    Psychology,
    Sociology,
    Theater,
    Theology,
    Transportation,
}
