use yew::{Component,Context,html,Html,InputEvent,TargetCast};
use web_sys::HtmlInputElement;
use js_sys::Date;
use primal::StreamingSieve;

#[derive(Default)]
struct Prime{
    n: usize,
    prime: usize,
    run_time: f64,
}

enum Message{
    Inputted(String),
    Run,
}

impl Component for Prime{
    type Message=Message;
    type Properties=();

    fn create(_ctx: &Context<Self>)->Self{
        Self::default()
    }

    fn update(&mut self,_ctx: &Context<Self>,msg: Self::Message)->bool{
        match msg{
            Message::Inputted(n)=>{
                self.n=match n.trim().parse(){
                    Ok(ok)=>ok,
                    Err(_)=>usize::default(),
                };
                false
            }
            Message::Run=>{
                if self.n>0{
                    let start=Date::now();
                    self.prime=StreamingSieve::nth_prime(self.n);
                    self.run_time=Date::now()-start;
                }

                true
            }
        }
    }

    fn view(&self,ctx: &Context<Self>)->Html{
        html!{
            <div>
                <div>
                    <h1>{"Let's Witness the Nth Prime Number!"}</h1>
                </div>
                <div class="text_input_container">
                    <input type="text" id="nth_input" oninput={ctx.link().callback(|e: InputEvent| Message::Inputted(e.target_unchecked_into::<HtmlInputElement>().value()))} />
                    <label for="nth_input">{"write the N here!"}</label>
                </div>
                <div id="button_container">
                    <button class="Run" onclick={ctx.link().callback(|_| Message::Run)}>
                        <span class="circle" >
                            <span class="icon arrow"></span>
                        </span>
                        <span class="button_text">{"Run"}</span>
                    </button>
                </div>
                <div>
                    <div class="prime">
                        if (self.prime)!=usize::default(){
                            {self.prime}
                        }
                    </div>
                </div>
                <div>
                    <p class="run_time">{format!("run time: {} msec",self.run_time)}</p>
                </div>
            </div>
        }
    }
}

fn main(){
    yew::start_app::<Prime>();
}