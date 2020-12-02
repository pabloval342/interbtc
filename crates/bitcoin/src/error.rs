#[derive(Debug)]
pub enum Error {
    MalformedMerkleProof,
    InvalidMerkleProof,
    EOS,
    MalformedHeader,
    MalformedTransaction,
    UnsupportedInputFormat,
    MalformedWitnessOutput,
    MalformedP2PKHOutput,
    MalformedP2SHOutput,
    UnsupportedOutputFormat,
    MalformedOpReturnOutput,
    InvalidHeaderSize,
    InvalidBtcHash,
    InvalidScript,
    InvalidBtcAddress,
}
