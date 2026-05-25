use signal_frame::{LogVariant, ShortHeader};

signal_frame::emit_schema!("owner-spirit.schema");

#[test]
fn owner_schema_generates_permissioned_owner_leg_routes() {
    assert_eq!(owner_spirit::ROUTE_COUNT, 5);
    assert!(
        owner_spirit::ROUTES
            .iter()
            .all(|route| route.leg == owner_spirit::Leg::Owner)
    );

    let operation = owner_spirit::OwnerOperation::Owner(owner_spirit::OwnerEndpoint::Register(
        owner_spirit::Registration(owner_spirit::IdentityName("author".to_string())),
    ));

    assert_eq!(
        operation.log_variant(),
        u64::from_le_bytes([0, 3, 0, 0, 0, 0, 0, 0])
    );

    let route = owner_spirit::route_for_short_header(
        owner_spirit::Leg::Owner,
        ShortHeader::new(operation.log_variant()),
    )
    .expect("owner register route");

    assert_eq!(route.root, "Owner");
    assert_eq!(route.endpoint, "Register");
    assert_eq!(
        route.body,
        owner_spirit::RouteBodyDescriptor::Type("Registration")
    );
}

#[test]
fn owner_schema_generates_lifecycle_reply_surface() {
    let reply = owner_spirit::Reply::Started(owner_spirit::Started(owner_spirit::Generation(1)));

    assert_eq!(
        reply,
        owner_spirit::Reply::Started(owner_spirit::Started(owner_spirit::Generation(1)))
    );
}
