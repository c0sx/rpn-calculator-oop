#[derive(Debug)]
pub struct RpnTokens {
    tokens: Vec<String>
}

impl RpnTokens {
    pub fn from(tokens: Vec<String>) -> RpnTokens {
        RpnTokens {
            tokens
        }
    }
}
