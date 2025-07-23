pub mod tension;
pub mod corruption;
pub mod homeostasis;
pub mod relation;
pub mod schema;
pub mod juizos;
pub mod extension;
pub mod flux;

#[macro_export]
macro_rules! tension {
    ($name:expr, $ty:ty, $rule:expr, $msg:expr) => {
        pot_philosophical::tension::Tension::new($name, $rule, $msg)
    };
}

#[macro_export]
macro_rules! homeostasis {
    ($name:expr, $ty:ty, $func:expr) => {
        pot_philosophical::homeostasis::DynHomeostasis::new($name, $func)
    };
}

#[macro_export]
macro_rules! relation {
    ($desc:expr, $func:expr) => {
        pot_philosophical::relation::Relation::new($desc, $func)
    };
}

#[macro_export]
macro_rules! ternary {
    ($prefix:expr, $ty:ty, $rule:expr, $msg:expr) => {
        (
            pot_philosophical::tension::Tension::new(
                concat!("archetype_", $prefix),
                $rule,
                $msg
            ),
            pot_philosophical::tension::Tension::new(
                concat!("manifestation_", $prefix),
                $rule,
                $msg
            ),
            pot_philosophical::tension::Tension::new(
                concat!("form_", $prefix),
                $rule,
                $msg
            ),
        )
    };
}

#[macro_export]
macro_rules! dialetic_schema {
    ($schema:ident, $tese:block, $antitese:block, $sintese:block) => {
        {
            // Initialize flux recorder
            let mut flux = pot_philosophical::flux::Flux::new();
            // Tese
            let mut $schema = { $tese };
            flux.record(&$schema);
            // Antítese
            { $antitese }
            flux.record(&$schema);
            // Homeostase intermediária
            $schema.apply_homeostasis();
            flux.record(&$schema);
            // Síntese
            { $sintese }
            flux.record(&$schema);
            ($schema, flux)
        }
    };
}