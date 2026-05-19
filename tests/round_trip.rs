use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_persona_spirit::{
    BootstrapPolicyReloaded, DrainAndStopOrder, DrainedAndStopped, Frame, FrameBody, Generation,
    IdentityName, IdentityRegistered, IdentityRetired, OperationKind, OwnerSpiritReply,
    OwnerSpiritRequest, RegisterIdentity, ReloadBootstrapPolicyOrder, RequestUnimplemented,
    RetireIdentity, StartOrder, Started, UnimplementedReason,
};
use signal_core::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalVerb, SubReply,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn psyche() -> IdentityName {
    IdentityName::new("author")
}

fn round_trip_request(request: OwnerSpiritRequest) -> OwnerSpiritRequest {
    let expected_verb = request.signal_verb();
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => {
            let operation = request.operations().head();
            assert_eq!(operation.verb, expected_verb);
            operation.payload.clone()
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: OwnerSpiritReply) -> OwnerSpiritReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::completed(NonEmpty::single(SubReply::Ok {
            verb: SignalVerb::Assert,
            payload: reply,
        })),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota text");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = T::decode(&mut decoder).expect("decode nota text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn owner_spirit_requests_round_trip() {
    let requests = [
        OwnerSpiritRequest::StartOrder(StartOrder {
            generation: Generation::new(1),
        }),
        OwnerSpiritRequest::DrainAndStopOrder(DrainAndStopOrder {}),
        OwnerSpiritRequest::ReloadBootstrapPolicyOrder(ReloadBootstrapPolicyOrder {}),
        OwnerSpiritRequest::RegisterIdentity(RegisterIdentity { name: psyche() }),
        OwnerSpiritRequest::RetireIdentity(RetireIdentity { name: psyche() }),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn owner_spirit_replies_round_trip() {
    let replies = [
        OwnerSpiritReply::Started(Started {
            generation: Generation::new(1),
        }),
        OwnerSpiritReply::DrainedAndStopped(DrainedAndStopped {}),
        OwnerSpiritReply::BootstrapPolicyReloaded(BootstrapPolicyReloaded {}),
        OwnerSpiritReply::IdentityRegistered(IdentityRegistered { name: psyche() }),
        OwnerSpiritReply::IdentityRetired(IdentityRetired { name: psyche() }),
        OwnerSpiritReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::StartOrder,
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn owner_spirit_request_variants_declare_expected_signal_root_verbs() {
    let cases = [
        (
            OwnerSpiritRequest::StartOrder(StartOrder {
                generation: Generation::new(1),
            }),
            SignalVerb::Mutate,
        ),
        (
            OwnerSpiritRequest::DrainAndStopOrder(DrainAndStopOrder {}),
            SignalVerb::Mutate,
        ),
        (
            OwnerSpiritRequest::ReloadBootstrapPolicyOrder(ReloadBootstrapPolicyOrder {}),
            SignalVerb::Mutate,
        ),
        (
            OwnerSpiritRequest::RegisterIdentity(RegisterIdentity { name: psyche() }),
            SignalVerb::Mutate,
        ),
        (
            OwnerSpiritRequest::RetireIdentity(RetireIdentity { name: psyche() }),
            SignalVerb::Retract,
        ),
    ];

    for (request, verb) in cases {
        assert_eq!(request.signal_verb(), verb);
    }
}

#[test]
fn owner_spirit_request_exposes_contract_owned_operation_kind() {
    assert_eq!(
        OwnerSpiritRequest::StartOrder(StartOrder {
            generation: Generation::new(1),
        })
        .operation_kind(),
        OperationKind::StartOrder
    );
    assert_eq!(
        OwnerSpiritRequest::RetireIdentity(RetireIdentity { name: psyche() }).operation_kind(),
        OperationKind::RetireIdentity
    );
}

#[test]
fn owner_spirit_canonical_examples_round_trip() {
    round_trip_nota(
        OwnerSpiritRequest::StartOrder(StartOrder {
            generation: Generation::new(1),
        }),
        "(StartOrder ((1)))",
    );
    round_trip_nota(
        OwnerSpiritRequest::RegisterIdentity(RegisterIdentity { name: psyche() }),
        "(RegisterIdentity (author))",
    );
    round_trip_nota(
        OwnerSpiritReply::Started(Started {
            generation: Generation::new(1),
        }),
        "(Started ((1)))",
    );
    round_trip_nota(
        OwnerSpiritReply::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::StartOrder,
            reason: UnimplementedReason::NotBuiltYet,
        }),
        "(RequestUnimplemented (StartOrder NotBuiltYet))",
    );
}
