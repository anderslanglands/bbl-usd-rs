use bbl_usd::usd::{self, Attribute, Object, PropertyKind};
use glam::{Vec2, Vec3, Vec4, Mat2, DMat2, Mat3, DMat3, Mat4, DMat4};

pub fn main() {
    let stage = usd::Stage::open("bbl-usd/test01.usda").expect("failed to open stage");
    println!("opened stage");

    let root = stage.pseudo_root();
    for prim in usd::PrimRange::from_prim(&root) {
        println!("{}", prim.path().text());

        for prop in prim.properties().iter() {
            match prop.property_kind() {
                PropertyKind::Attribute(attr) => {
                    print_attribute(&attr);
                }
                PropertyKind::Relationship(rel) => {
                    if let Some(targets) = rel.targets() {
                        let contents = targets
                            .iter()
                            .map(|t| format!("<{}>", t.text()))
                            .collect::<Vec<String>>()
                            .join(", ");
                        println!("  {} = [{}]", rel.name().text(), contents);
                    } else {
                        println!("  {} = []", rel.name().text());
                    };
                }
            }
        }
    }
}

fn print_attribute(attr: &Attribute) {
    if let Some(value) = attr.get() {
        if let Some(token) = value.as_token() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                token
            );
        } else if let Some(val) = value.as_asset_path() {
            let path = if val.resolved_path().is_empty() {
                val.asset_path()
            } else {
                val.resolved_path()
            };

            println!(
                "  {}: {} = @{}@",
                attr.name().text(),
                attr.type_name().as_token(),
                path
            );
        } else if let Some(val) = value.get::<i32>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<f32>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<f64>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<bool>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<Vec2>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<Vec3>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<Vec4>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<Mat2>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<DMat2>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<Mat3>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<DMat3>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<Mat4>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.get::<DMat4>() {
            println!(
                "  {}: {} = {}",
                attr.name().text(),
                attr.type_name().as_token(),
                val
            );
        } else if let Some(val) = value.as_token_array() {
            println!(
                "  {}: {} = [{}]",
                attr.name().text(),
                attr.type_name().as_token(),
                val.iter()
                    .map(|x| x.text())
                    .collect::<Vec<&str>>()
                    .join(", ")
            );
        } else if let Some(val) = value.as_int_array() {
            let size = val.size();
            let elide = if size < 8 { "" } else { "..." };
            println!(
                "  {}: {} = {}:[{}{}]",
                attr.name().text(),
                attr.type_name().as_token(),
                size,
                val.iter()
                    .take(size.min(8))
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                elide
            );
        } else if let Some(val) = value.as_float_array() {
            let size = val.size();
            let elide = if size < 8 { "" } else { "..." };
            println!(
                "  {}: {} = {}:[{}{}]",
                attr.name().text(),
                attr.type_name().as_token(),
                size,
                val.iter()
                    .take(size.min(8))
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                elide
            );
        } else if let Some(val) = value.as_double_array() {
            let size = val.size();
            let elide = if size < 8 { "" } else { "..." };
            println!(
                "  {}: {} = {}:[{}{}]",
                attr.name().text(),
                attr.type_name().as_token(),
                size,
                val.iter()
                    .take(size.min(8))
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                elide
            );
        } else if let Some(val) = value.as_vec2_array() {
            let size = val.size();
            let elide = if size < 5 { "" } else { "..." };
            println!(
                "  {}: {} = {}:[{}{}]",
                attr.name().text(),
                attr.type_name().as_token(),
                size,
                val.iter()
                    .take(size.min(5))
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                elide
            );
        } else if let Some(val) = value.as_vec3_array() {
            let size = val.size();
            let elide = if size < 5 { "" } else { "..." };
            println!(
                "  {}: {} = {}:[{}{}]",
                attr.name().text(),
                attr.type_name().as_token(),
                size,
                val.iter()
                    .take(size.min(5))
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                elide
            );
        } else if let Some(val) = value.as_vec4_array() {
            let size = val.size();
            let elide = if size < 5 { "" } else { "..." };
            println!(
                "  {}: {} = {}:[{}{}]",
                attr.name().text(),
                attr.type_name().as_token(),
                size,
                val.iter()
                    .take(size.min(5))
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                elide
            );
        } else {
            println!("  {}: {}", attr.name().text(), attr.type_name().as_token());
        }
    } else {
        println!("  {}: {}", attr.name().text(), attr.type_name().as_token());
    }
}
