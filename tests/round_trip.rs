use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_persona_spirit::{
    BootstrapPolicy, BootstrapPolicyReloaded, Drain, DrainedAndStopped, Frame, FrameBody,
    Generation, IdentityName, IdentityRegistered, IdentityRetired, Operation, Registration, Reply,
    RequestUnimplemented, Retirement, Start, Started, UnimplementedReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SignalOperationHeads, SubReply,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn registration() -> Registration {
    Registration {
        name: IdentityName::new("author"),
    }
}

fn retirement() -> Retirement {
    Retirement {
        name: IdentityName::new("author"),
    }
}

fn round_trip_request(request: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: FrameReply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota text");
    let encoded = encoder.into_string();
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = Value::decode(&mut decoder).expect("decode nota text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn owner_spirit_requests_round_trip() {
    let requests = [
        Operation::Start(Start {
            generation: Generation::new(1),
        }),
        Operation::Drain(Drain {}),
        Operation::Reload(BootstrapPolicy {}),
        Operation::Register(registration()),
        Operation::Retire(retirement()),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn owner_spirit_replies_round_trip() {
    let replies = [
        Reply::Started(Started {
            generation: Generation::new(1),
        }),
        Reply::DrainedAndStopped(DrainedAndStopped {}),
        Reply::BootstrapPolicyReloaded(BootstrapPolicyReloaded {}),
        Reply::IdentityRegistered(IdentityRegistered {
            name: IdentityName::new("author"),
        }),
        Reply::IdentityRetired(IdentityRetired {
            name: IdentityName::new("author"),
        }),
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn owner_spirit_reply_payloads_convert_through_macro_generated_from_impls() {
    let reply: Reply = Started {
        generation: Generation::new(1),
    }
    .into();

    assert_eq!(
        reply,
        Reply::Started(Started {
            generation: Generation::new(1),
        })
    );
}

#[test]
fn owner_spirit_request_variants_are_contract_local_verbs() {
    assert_eq!(
        Operation::HEADS,
        &["Start", "Drain", "Reload", "Register", "Retire"]
    );
}

#[test]
fn owner_spirit_request_heads_have_no_universal_verb_wrapper() {
    round_trip_nota(
        Operation::Start(Start {
            generation: Generation::new(1),
        }),
        "(Start (1))",
    );
    round_trip_nota(Operation::Drain(Drain {}), "(Drain ())");
    round_trip_nota(Operation::Reload(BootstrapPolicy {}), "(Reload ())");
    round_trip_nota(Operation::Register(registration()), "(Register (author))");
    round_trip_nota(Operation::Retire(retirement()), "(Retire (author))");
}

#[test]
fn owner_spirit_canonical_examples_round_trip() {
    round_trip_nota(
        Reply::Started(Started {
            generation: Generation::new(1),
        }),
        "(Started (1))",
    );
    round_trip_nota(
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
        "(RequestUnimplemented (NotBuiltYet))",
    );
}
