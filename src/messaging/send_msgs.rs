
use std::{collections::HashSet, io};

use tokio::{ io::AsyncWriteExt, net::TcpStream};
use uuid::Uuid;

use crate::models::{Message, MessageType, Node, State};

impl Node {
    async fn send_msg(&self,message:&Message,reveiver_addr: &str) -> io::Result<()>{

        let mut stream = TcpStream::connect(reveiver_addr).await?;

        let serialized_msg = serde_json::to_vec(message)?;
        stream.write_all(&serialized_msg).await
        
    }

    async fn broadcast_proposal(&self,proposed_state: State) {
        
        let proposal_id = Uuid::new_v4().to_string();
        let message = Message{
            sender_id : self.id,
            proposed_state: proposed_state,
            message_type : MessageType::Proposal,
            proposal_id: proposal_id.clone(),

        };

        let mut proposal_ack = self.proposal_ack.lock().await;
        proposal_ack.insert(proposal_id.clone(), HashSet::new());
        for address in self.peers.values(){
            if let Err(e) = self.send_msg(&message, address).await {
                eprintln!("Failed to send message")
            }
        }
        //TODO : self.wait_for_acknowledgments(proposal_id).await;

    }
}