#[test]
fn serialize() {
    let inline = jarkup_rs::InlineComponent::Text(jarkup_rs::Text {
        props: jarkup_rs::TextProps {
            text: "Hello, world!".to_string(),
            ..Default::default()
        },
        ..Default::default()
    });

    let paragraph = jarkup_rs::Component::BlockComponent(jarkup_rs::BlockComponent::Paragraph(
        jarkup_rs::Paragraph {
            props: None,
            id: None,
            slots: jarkup_rs::ParagraphSlots {
                default: vec![inline],
            },
        },
    ));

    println!("{}", serde_json::to_string(&paragraph).unwrap());
}

#[test]
fn deserialize() {
    let slice = include_bytes!("./seed.json").as_slice();

    let results = serde_json::from_slice::<Vec<jarkup_rs::Component>>(slice).unwrap();

    println!("{}", serde_json::to_string(&results).unwrap());
}
