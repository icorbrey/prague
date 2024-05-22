pub struct ProduceApi;

#[obake::versioned]
#[obake(version("0.0.0"))]
#[obake(version("3.0.0"))]
#[derive(Debug, PartialEq, Eq)]
pub struct ProduceRequest {
	#[obake(cfg(">=3"))]
	pub transactional_id: Option<String>,
	
    #[obake(cfg(">=0"))]
    pub acks: i16,

    #[obake(cfg(">=0"))]
    pub timeout: i32,

    #[obake(cfg(">=0"))]
    pub topics: Vec<Topic>,
}

#[obake::versioned]
#[obake(version("0.0.0"))]
#[derive(Debug, PartialEq, Eq)]
pub struct Topic {
    #[obake(cfg(">=0"))]
    pub name: String,

    #[obake(cfg(">=0"))]
	pub partitions: Vec<Partition>,
}

#[obake::versioned]
#[obake(version("0.0.0"))]
#[derive(Debug, PartialEq, Eq)]
pub struct Partition {
    #[obake(cfg(">=0"))]
	pub index: i32,
	
    #[obake(cfg(">=0"))]
	pub records: Vec<()>,
}

pub struct ProduceResponse;
