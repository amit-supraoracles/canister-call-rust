use ic_agent::{Agent, agent::{http_transport::ReqwestHttpReplicaV2Transport}, ic_types::Principal};

#[tokio::main]
async fn main() {
    // let config = AgentConfig::default();
    let url = "http://127.0.0.1:8000".to_string();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();

    let agent = Agent::builder().with_transport(transport).build().unwrap();

    let canister_id = Principal::from_text("qvhpv-4qaaa-aaaaa-aaagq-cai").unwrap();
    let method_name = "print".to_string();

    let response = agent.query(&canister_id, method_name).call().await;

    if response.is_ok() {
        println!("response:{:#?}",response.unwrap());
    } else {
        let error = response.unwrap_err();
        println!("error response:{:#?}",error);
    }
    
   
}
