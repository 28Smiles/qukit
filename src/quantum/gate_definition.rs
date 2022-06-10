
pub enum GateTypes {
    X(PauliX),
    Y,
    Z,
    H,
    T,
    RX,
    RY,
    RZ,
    CX,
    CY,
    CZ,
    CH,
    CRX,
    CRY,
    CRZ,
}

pub struct GateDefinition {
    id: GateTypes,

}
