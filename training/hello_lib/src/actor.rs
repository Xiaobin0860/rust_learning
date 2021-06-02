// Actor
// ActorMessage
// HandleCall
// Pid
use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

pub struct Actor<State, Request, Reply> {
    receiver: mpsc::Receiver<ActorMessage<Request, Reply>>,
    state: State,
}

impl<State, Request, Reply> Actor<State, Request, Reply>
where
    State: HandleCall<Request = Request, Reply = Reply> + Send + 'static,
    Request: Send + 'static,
    Reply: Send + 'static,
{
    pub fn spawn(max_msg_len: usize, state: State) -> Result<Pid<Request, Reply>> {
        let (sender, receiver) = mpsc::channel(max_msg_len);

        let mut actor = Self {
            receiver,
            state: state,
        };

        tokio::spawn(async move {
            while let Some(msg) = actor.receiver.recv().await {
                let state = &mut actor.state;
                let reply = state.handle_call(&msg.data).unwrap();
                let _ = msg.sender.send(reply);
            }
        });

        Ok(Pid { sender })
    }
}

struct ActorMessage<Request, Reply> {
    sender: oneshot::Sender<Reply>,
    data: Request,
}

#[derive(Debug, Clone)]
pub struct Pid<Request, Reply> {
    sender: mpsc::Sender<ActorMessage<Request, Reply>>,
}

impl<Request, Reply> Pid<Request, Reply> {
    pub async fn send(&self, data: Request) -> Result<Reply> {
        let (sender, receiver) = oneshot::channel();
        let msg = ActorMessage { sender, data };
        let _ = self.sender.send(msg).await;
        Ok(receiver.await?)
    }
}

pub trait HandleCall {
    type Request;
    type Reply;

    fn handle_call(&mut self, request: &Self::Request) -> Result<Self::Reply>;
}

#[cfg(test)]
mod tests {
    use super::*;

    impl HandleCall for i32 {
        type Request = &'static str;
        type Reply = i32;

        fn handle_call(&mut self, request: &Self::Request) -> Result<Self::Reply> {
            match request {
                &cmd @ "+1" => {
                    *self += 1;
                    println!("recv cmd: {}, state={}", cmd, *self);
                    Ok(*self)
                }
                &cmd @ "-1" => {
                    *self -= 1;
                    println!("recv cmd: {}, state={}", cmd, *self);
                    Ok(*self)
                }
                &_ => unimplemented!(),
            }
        }
    }

    #[tokio::test]
    async fn it_works() {
        let p: Pid<&str, i32> = Actor::spawn(10, 0i32).unwrap();
        let r = p.send("+1").await.unwrap();
        assert_eq!(r, 1);
        let p2 = p.clone();
        let r2 = p2.send("+1").await.unwrap();
        assert_eq!(r2, 2);
        let p3 = p.clone();
        let r3 = p3.send("-1").await.unwrap();
        assert_eq!(r3, 1);
    }
}
