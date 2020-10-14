use actix::prelude::*;
use actix::dev::{MessageResponse, ResponseChannel};


#[derive(Message)]
#[rtype(result="Responses")]
enum Messages {
    Ping,
    Pong,
}


enum Responses{
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Responses
    where
        A: Actor,
        M: Message<Result = Responses>,
    {
        fn handle<R: ResponseChannel<M>>(self, ctx: &mut <A as Actor>::Context, tx: Option<R>) {
            if let Some(tx) = tx {
                tx.send(self);
            }
        }
    }

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor stopped");
    }


}

impl Handler<Messages> for MyActor {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Messages::Ping => Responses::GotPing,
            Messages::Pong => Responses::GotPong,
        }
    }
}

#[actix_rt::main]
async fn main() {
    let addr = MyActor.start();

    let ping_future = addr.send(Messages::Ping).await;
    let pong_future = addr.send(Messages::Pong).await;

    match pong_future {
        Ok(res) => match res {
            Responses::GotPing => println!("Ping received for pong_future"),
            Responses::GotPong => println!("Pong received for pong_future"),
        },
        Err(e) => println!("Actor is probably dead: {}", e),
    }

    match ping_future {
        Ok(res) => match res {
            Responses::GotPing => println!("Ping received for ping_future"),
            Responses::GotPong => println!("Pong received for ping_future"),
        },
        Err(e) => println!("Actor is probably dead: {}", e),
    }

}
