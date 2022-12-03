use macroquad::prelude::*;
use macroquad::audio::{play_sound_once,load_sound};

#[macroquad::main("Clicker Game")]
async fn main() {
   let (x,y) = (screen_width()/2.,screen_height()/2.);
   let r = 70.;
   let circle = Circle::new(x,y,r);
   let mut score = 0;
   let click_sound = load_sound("res/click.wav").await.unwrap();
   let milestone_sound = load_sound("res/milestone.wav").await.unwrap();
   loop {
      clear_background(GRAY);

      if is_mouse_button_pressed(MouseButton::Left) {
         let (mouse_x,mouse_y) = mouse_position();
         let mouse_circ = Circle::new(mouse_x,mouse_y,1.);
         
         if  circle.overlaps(&mouse_circ){
            score+=1;
            play_sound_once(click_sound);
            
            if score%10 ==0 {
               play_sound_once(milestone_sound);
            }

         }
      }

      draw_text("Clicker Game",screen_width()/2.-100.,100.,50.,WHITE);
      draw_text(format!("Clicks: {}",score).as_str(),screen_width()/2.-100.,500.,50.,WHITE);
      draw_circle(x,y,r,RED);
      next_frame().await;
   }
}
