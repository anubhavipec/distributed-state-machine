use std::{collections::{HashMap, HashSet}, sync::{Arc}};

use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, Mutex};




#[derive(Eq,PartialEq,Debug,Clone,Hash,Serialize,Deserialize)]
pub enum State {
    Init,
    Running,
    Stopped,
}

#[derive(Serialize,Deserialize,Debug)]
pub enum MessageType{
    Proposal,
    Acknowledgment,
    Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message{

    pub sender_id: u64,
    pub message_type: MessageType,
    pub proposed_state: State,
    pub proposal_id: String,
}

pub struct Node {
    pub id: u64,                            // Make id public
    pub state: Arc<Mutex<State>>,           // Make state public
    pub peers: HashMap<u64, String>,        // Make peers public
    pub address: String,                     // Make address public
    pub tx: mpsc::Sender<String>,            // Make tx public
    pub proposal_ack: Arc<Mutex<HashMap<String,HashSet<u64>>>>,
}