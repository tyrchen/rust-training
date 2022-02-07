use prost::Message;
use prost_live::pb::*;

fn main() {
    let phones = vec![PhoneNumber::new("111-222-3334", PhoneType::Mobile)];
    let person = Person::new("Tyr Chen", 1, "tyr@awesome.com", phones);
    let v1 = person.encode_to_vec();
    let person1 = Person::decode(v1.as_ref()).unwrap();
    assert_eq!(person, person1);

    let json = serde_json::to_string_pretty(&person1).unwrap();

    println!("{}", json);
}
