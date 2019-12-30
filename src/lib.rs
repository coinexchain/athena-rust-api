mod athena {

    #[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Bytes20 {
        pub bytes: [u8; 20],
    }

    #[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Bytes32 {
        pub bytes: [u8; 32],
    }

    pub type Address = Bytes20; // AccAddress
    pub type BigInt = Bytes32; // sdk.Int
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
