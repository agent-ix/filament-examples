use filament_exact::{Integer, NodeKey, PathSegment, UnionMember, Value, ValueType, VariantId};
use filament_semantic_value::declaration::{
    CompositeDeclaration, CompositeShape, TypeEnvironment, UnionMemberDeclaration,
};

fn key() -> NodeKey {
    NodeKey::new("docs", vec![PathSegment::Text("Shape".into())], 0).unwrap()
}

fn member(name: &str, positions: Vec<ValueType>) -> UnionMemberDeclaration {
    UnionMemberDeclaration::from_verified(
        UnionMember::from_admitted(key(), name).unwrap(),
        positions,
    )
}

fn int(n: i64) -> Value {
    Value::Integer(Integer::from(n))
}

fn main() {
    // A union Shape with members Rect(Integer, Integer), Empty and Circle(Integer).
    let shape = CompositeDeclaration::new(
        key(),
        "Shape",
        CompositeShape::Union(vec![
            member("Rect", vec![ValueType::Integer, ValueType::Integer]),
            member("Empty", vec![]),
            member("Circle", vec![ValueType::Integer]),
        ]),
    );
    let env = TypeEnvironment::new([shape], []).unwrap();

    let (position, rect) = env.union_member_named(key(), "Rect").unwrap();
    println!("Rect is member {position}, positions {:?}", rect.positions());
    println!("lookup is case sensitive: {}", env.union_member_named(key(), "rect").is_none());

    let variant = VariantId::new(key(), "Rect").unwrap();
    let value = env.union(key(), variant.clone(), vec![int(2), int(3)]).unwrap();
    println!(
        "Rect(2, 3) admitted by Shape: {}",
        env.admits(&ValueType::Composite(key()), &value).unwrap()
    );

    // A payload with the wrong number of positions is refused when it is built.
    let refused = env.union(key(), variant, vec![int(2)]);
    println!("Rect(2) built: {}", refused.is_ok());
}
