use std::{time::{Duration, Instant}, fs::File, io::{BufReader, Read}};
use actix::prelude::*;
use actix_web_actors::ws;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


pub struct MyWebSocket{
    hb : Instant
}

/*------------------impl actix::Actor Trait for my custom MyWebSocket ------------------*/
impl Actor for MyWebSocket{
    type Context = ws::WebsocketContext<Self>;

    /// Start a HB process for this connection
    fn started(&mut self, ctx : &mut Self::Context){
        self.hb(ctx);
    }
}



impl MyWebSocket{

    pub fn new() -> Self{
        Self{hb : Instant::now()}
    }

    /*  
        this fn will run on an interval, every 5 sec to check that 
        the connection is still alive. If it's more than 10 seconds
        since last ping, it'll close the connection
    */
    fn hb(&self, ctx: &mut <Self as Actor>::Context){
        ctx.run_interval(HEARTBEAT_INTERVAL,|act, ctx|{
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT{
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }

}


/* ----------The StreamHandler trait is used to handle the message that are sent over the socket-------- */

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket{
    
    /* 
        The 'handle()' fn is where we'll determine the response to the client's message
        E.g: If we ping the client, it should response with pong.
        Which is necessory fot the 'hb()' fn to maintain the connection status
    */

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            
            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) =>{
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) =>{
                self.hb = Instant::now();
            }

            // Text will echo any text received back to the client (for now)
            Ok(ws::Message::Text(_)) => {
                let file = File::open("./static/test-50mb.bin").expect("Failed to get the test file");
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).expect("failed to read the test file");
                ctx.binary(buffer);
            },
            // Close will close the socket
            Ok(ws::Message::Close(reason))=>{
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}