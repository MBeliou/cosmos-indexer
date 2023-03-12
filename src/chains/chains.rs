#[derive(Debug, Clone, Copy)]
pub struct Chain {
    pub id: &'static str,
    pub name: &'static str,
    pub rpcs: &'static [&'static str],
    pub min_height: Option<u32>,
}

pub const FETCHHUB: Chain = Chain {
    id: "fetchhub-4",
    name: "fetchhub",
    rpcs: &["https://rpc-fetchhub.fetch.ai:443"],
    min_height: Some(5300201),
};

pub static CHAINS: [Chain; 1] = [FETCHHUB];

pub fn get_chain(id: String) -> Option<Chain> {
    CHAINS.into_iter().find(|c| c.id == id)
}
