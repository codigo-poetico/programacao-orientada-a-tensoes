use pot_philosophical::*;
fn main() {
    let mut schema = schema::Schema::new();
    // Define tensions
    schema.add_tension(tension!("idade", u8, |v| *v >= 18, "Idade inválida"));
    schema.add_tension(tension!("nome", String, |v| !v.is_empty(), "Nome vazio"));
    // Dialética completa
    let (final_schema, flux) = dialetic_schema!(schema, {
        let mut s = schema;
        s.set("idade", 20u8);
        s
    }, {
        let mut s = final_schema.clone();
        s.set("idade", 15u8);
        s
    }, {
        let mut s = final_schema.clone();
        s.set("idade", 25u8);
        s
    });
    println!("Flux history: {:#?}", flux);
}