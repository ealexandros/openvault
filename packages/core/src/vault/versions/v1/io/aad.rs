#[derive(Debug, Clone, Copy)]
pub enum AadDomain {
    Subheader,
    Checkpoint,
    Record,
    Payload,
}

pub fn encode_aad(domain: AadDomain, offset: u64) -> Vec<u8> {
    let mut aad = b"openvault/v1/".to_vec();

    aad.push(match domain {
        AadDomain::Subheader => 1,
        AadDomain::Checkpoint => 2,
        AadDomain::Record => 3,
        AadDomain::Payload => 4,
    });
    aad.extend_from_slice(&offset.to_le_bytes());

    aad
}
