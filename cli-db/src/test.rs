
enum InputType{
    Person(CustomStruct1),
    Address(CustomStruct2),
    EnumEnumStruct(CustomEnum)
}

struct CustomStruct1 {
    name: String,
    hobby: String,
}

struct CustomStruct2 {
    street: String,
    house_number: i64,
    postal_cd: i64,
    city: String
}

enum CustomEnum {
    Option1(CustomStruct1),
    Option2(CustomStruct2)
}

fn perform_action_on(input:InputType) {
    // Function takes as input any argument belonging to the InputType
    match input {
        InputType::Person(person) => { 
            println!("{} likes to {}", person.name, person.hobby);
        }
        InputType::Address(address) => {
            println!("Address: {} {},{} {}", address.street,
                        address.house_number, 
                        address.postal_cd, 
                        address.city );
        }
        InputType::EnumEnumStruct(custom_enum) => {
                match custom_enum {
                CustomEnum::Option1(person) => {println!("Hello")},
                CustomEnum::Option2(address) => {println!("World")}
            }
        }
    }
}

fn main(){

    let person = InputType::Person(CustomStruct1{
        name: "John".to_string(),
        hobby: "reading".to_string()
    });

    let address = InputType::Address(CustomStruct2{
        street: "some_street".to_string(),
        house_number: 77,
        postal_cd: 21314,
        city: "reading".to_string()
    });

    perform_action_on(person);
    perform_action_on(address);

    let custom_enum_input = InputType::EnumEnumStruct(CustomEnum::Option1(CustomStruct1 {
        name: "John".to_string(),
        hobby: "reading".to_string(),
    }));

    perform_action_on(custom_enum_input);

}