use std::time::Instant;
use macroquad::prelude::*;
use macroquad::texture::*;
use super::physics::*;
use super::camera::*;
use std::any::Any;

use super::texturetools::*;

#[derive(Clone)]
pub enum WindowType {
    Error,
    Alert,
}

pub struct VintageWindow {
    window_name: String,
    message_text: String,
    line_two: String,
    button_text: String,
    width: u16,
    height: u16,
    texture: Texture2D,
    window_type: WindowType,
    visible: bool,
}

impl VintageWindow {
    pub fn new(width: u16, height: u16, window_name: String, message_text: String, line_two: String, button_text: String, window_type: WindowType) -> VintageWindow {
        VintageWindow {
            window_name,
            message_text,
            line_two,
            button_text,
            width,
            height,
            texture: create_vintage_window_texture(width, height, &window_type),
            window_type,
            visible: false,
        }        
    }

    pub fn update(
        &mut self,
        quitting: &mut bool,
    ) {
        let draw_x = screen_width() / 2. - (self.width as f32 / 2.); 
        let draw_y = screen_height() / 2. - (self.height as f32 / 2.);


        let (mouse_x, mouse_y) = mouse_position(); 

        let button_width = (self.width as f32 * 0.28) as u32;
        let button_height = (self.height as f32 * 0.15) as u32;
        let top_left_x = (self.width as f32 * 0.5) as u32 - (button_width / 2);
        let top_left_y = (self.height as f32 * 0.7) as u32;

        // top left corner of the square
        let (x0, y0) = (self.width as f32 - self.width as f32 * 0.077, self.height as f32 * 0.025);

        let side_len = (self.width as f32 - self.width as f32 * 0.01) as u32 - (self.width as f32 - self.width as f32 * 0.07) as u32;
    

        if self.visible {

            if mouse_x as u32 >=(draw_x as u32 + top_left_x) 
                && mouse_y as u32 >=(draw_y as u32 + top_left_y) 
                && mouse_x as u32 <= (draw_x as u32 + top_left_x + button_width)
                && mouse_y as u32 <= (draw_y as u32 + top_left_y + button_height) 
                && is_mouse_button_pressed(MouseButton::Left)
            {
                self.visible = !self.visible;
                *quitting = true;
            }

            if mouse_x as u32 >= (draw_x as u32 + x0 as u32)
                && mouse_y as u32 >= (draw_y as u32 + y0 as u32)
                && mouse_x as u32 <= (draw_x as u32 + x0 as u32 + side_len)
                && mouse_y as u32 <= (draw_y as u32 + y0 as u32 + side_len)
                && is_mouse_button_pressed(MouseButton::Left) 
            {
                self.visible = false;
            }

            
        }

        if is_key_pressed(KeyCode::Escape) {
            self.visible = !self.visible;
        }
    }

    pub fn draw(
        &mut self,
    ) {
       if self.visible {
            let draw_x = screen_width() / 2. - (self.width as f32 / 2.); 
            let draw_y = screen_height() / 2. - (self.height as f32 / 2.);

            draw_texture_ex(
                &self.texture,
                draw_x,
                draw_y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                }
            );
            
            let font_size = self.height as f32 * 0.12;

            let button_width = (self.width as f32 * 0.28) as u32;
            let button_height = (self.height as f32 * 0.15) as u32;
            let top_left_x = (self.width as f32 * 0.5) - (button_width / 2) as f32;
            let top_left_y = (self.height as f32 * 0.7) as u32 as f32;

            draw_text_ex(
                self.window_name.as_str(),
                draw_x + (self.width as f32 * 0.015),
                draw_y + (self.height as f32 * 0.11),
                TextParams{
                    font_size: font_size as u16,
                    ..Default::default()
                }
            );

            draw_text_ex(
                self.button_text.as_str(),
                draw_x + top_left_x + (button_width as f32 * 0.37),
                draw_y + top_left_y + (button_height as f32 * 0.01 + font_size * 0.9),
                TextParams{
                    font_size: font_size as u16,
                    color: BLACK,
                    ..Default::default()
                }
            );


            draw_text_ex(
                self.message_text.as_str(),
                draw_x + (self.width as f32 * 0.3),
                draw_y + (self.height as f32 * 0.5),
                TextParams{
                    font_size: font_size as u16,
                    color: BLACK,
                    ..Default::default()
                }
            );

            draw_text_ex(
                self.line_two.as_str(),
                draw_x + (self.width as f32 * 0.3) + (self.width as f32 * 0.1),
                draw_y + (self.height as f32 * 0.5) + (font_size),
                TextParams{
                    font_size: font_size as u16,
                    color: BLACK,
                    ..Default::default()
                }
            );
        } 
    }
}

pub fn create_vintage_window_texture(width: u16, height: u16, window_type: &WindowType) -> Texture2D {

    let gray_color: Color = Color {
        r: 0.83921,
        g: 0.82745,
        b: 0.80784,
        a: 1.0,
    };
    
    let mut base_img_texture = Image::gen_image_color(width, height, gray_color);

    draw_base_window(width as u32, height as u32, &mut base_img_texture);
    // add type-specific graphic
    match window_type {
        WindowType::Error => {
            draw_red_x(width as u32, height as u32, &mut base_img_texture);
            draw_interactable_button(width as u32, height as u32, &mut base_img_texture);
        },
        WindowType::Alert => {

        },
    }
    
    draw_close_button(width as u32, height as u32, &mut base_img_texture);

    Texture2D::from_image(&base_img_texture)
}

fn draw_base_window(width: u32, height: u32, image: &mut Image) {

    let clear_color: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };

    let blue_color: Color = Color {
        r: 0.031372,
        g: 0.141176,
        b: 0.419607,
        a: 1.0,
    };

    for w in 0..width {
        for h in 0..height {
//        if ((w  as f32) < width as f32 * 0.01) || (w as f32 >= width as f32 - (width as f32 * 0.01)) {
//                image.set_pixel(w as u32, h as u32, blue_color);
//            }
        if (h as f32) < height as f32 * 0.15 || h as f32 > height as f32 - (width as f32 * 0.01) {
                image.set_pixel(w as u32, h as u32, blue_color);
            }
        }

    } 
/*
    // round off left corner
    let r = width as f32 * 0.02;
    for w in 0..r as i32 {
        for h in 0..r as i32 {
            let dx = w as f32 - r;
            let dy = h as f32 - r;

            let d2 = (dx * dx) + (dy * dy);
            if d2 > (r * r) {
               image.set_pixel(w as u32, h as u32, clear_color);
            }
        }
    }

    // round off right corner
    for w in ((width as f32 - r) as i32)..width as i32 {
        for h in 0..r as i32 {
            let dx = (w as f32 - (width as f32 - r));
            let dy = h as f32 - r;

            let d2 = (dx * dx) + (dy * dy);
            if d2 > (r * r) {
                image.set_pixel(w as u32, h as u32, clear_color);
            }
        }
    }
*/
}

fn draw_interactable_button(width: u32, height: u32, image: &mut Image) {
    let line_color: Color = Color { 
        r: 0.121568,
        g: 0.121568,
        b: 0.121568,
        a: 1.0,
    };
    
    let button_width = (width as f32 * 0.28) as u32;
    let button_height = (height as f32 * 0.15) as u32;
    let top_left_x = (width as f32 * 0.5) as u32 - (button_width / 2);
    let top_left_y = (height as f32 * 0.7) as u32;

    draw_image_line(top_left_x, top_left_y, top_left_x + button_width, top_left_y, 3, image, line_color);
    draw_image_line(top_left_x, top_left_y, top_left_x, top_left_y + button_height, 3, image, line_color);
    draw_image_line(top_left_x, top_left_y + button_height, top_left_x + button_width, top_left_y + button_height, 3, image, line_color);
    draw_image_line(top_left_x + button_width, top_left_y, top_left_x + button_width, top_left_y + button_height, 3, image, line_color);
}

fn draw_close_button(width: u32, height: u32, image: &mut Image) {

    let red_color: Color = Color {
        r: 0.99607,
        g: 0.0,
        b: 0.00784,
        a: 1.0,
    };

    let gray_color: Color = Color {
        r: 0.83921,
        g: 0.82745,
        b: 0.80784,
        a: 1.0,
    };
 
    // top left corner of the square
    let (x0, y0) = (width as f32 - width as f32 * 0.077, height as f32 * 0.025);

    let side_len = (width as f32 - width as f32 * 0.01) - (width as f32 - width as f32 * 0.07);
    
    // fill in red
    for w in (x0 as u32)..=(x0 as u32 + side_len as u32) {
        for h in (y0 as u32)..=(y0 as u32 + side_len as u32) {
            image.set_pixel(w, h, gray_color);
        }
    }
    let (p1, p2, p3, p4) = (x0 as u32 + (side_len * 0.25) as u32, y0 as u32 + (side_len * 0.25) as u32, x0 as u32 + side_len as u32 - (side_len * 0.25) as u32, y0 as u32 + side_len as u32 - (side_len * 0.25) as u32);
    // draw 'X'
    draw_image_line(p1, p2, p3, p4, 1, image, BLACK);
    draw_image_line(p1, p4, p3, p2, 1, image, BLACK);

    
    // outline in black
    for w in (x0 as u32)..=(x0 as u32 + side_len as u32) {
        for h in (y0 as u32)..=(y0 as u32 + side_len as u32) {
            if w == x0 as u32 || w == x0 as u32 + side_len as u32 {
                image.set_pixel(w, h, BLACK);
            }
            if h == y0 as u32 || h == y0 as u32 + side_len as u32 {
                image.set_pixel(w, h, BLACK);
            }
        }
    }

    // left and top need this color:
    let highlight_col: Color = Color {
        r: 1.0,
        g: 0.99215,
        b: 0.97254,
        a: 1.0,
    };

    for w in 0..3 {
        for h in 0..height {
            image.set_pixel(w, h, highlight_col);
        }
    }

    for w in 0..width {
        for h in 0..3 {
            image.set_pixel(w, h, highlight_col);
        }
    }

    // add this semi-shadow:
    let semi_shadow: Color = Color {
        r: 0.80392,
        g: 0.80392,
        b: 0.77254,
        a: 1.0,
    };
    for w in 0..width {
        image.set_pixel(w, 1, semi_shadow);
    }
    for h in 0..height {
        image.set_pixel(1, h, semi_shadow);
    }

    // bottom and right need this color:
    let shadow_col: Color = Color {
        r: 0.25490,
        g: 0.25038,
        b: 0.266666,
        a: 1.0,
    };

    for w in 0..width {
        for h in height -3 .. height {
            image.set_pixel(w, h, shadow_col);
        }
    }

    for w in width -3 .. width {
        for h in 0..height {
            image.set_pixel(w, h, shadow_col);
        }
    }

}

fn draw_red_x(width: u32, height: u32, image: &mut Image) {

    let red_color: Color = Color {
        r: 0.99607,
        g: 0.0,
        b: 0.00784,
        a: 1.0,
    };


    let (circle_x, circle_y) = (width as f32 * 0.15, height as f32 * 0.5);
    let r = height as f32 * 0.15;

    for w in (circle_x - r) as i32..(circle_x + r) as i32 {
        for h in (circle_y - r) as i32..(circle_y + r) as i32 {
            let dx = circle_x - w as f32;
            let dy = circle_y - h as f32;
            let d = ((dx * dx) + (dy * dy)).sqrt();

            if d < r {
                image.set_pixel(w as u32, h as u32, red_color);
            }
        } 
        
    }

    let (p1x, p1y) = (
        circle_x as f64 + r as f64 * (std::f64::consts::PI / 4.).cos(),
        circle_y as f64 + r as f64 * (std::f64::consts::PI / 4.).sin()
    );

    let (p2x, p2y) = (
        circle_x as f64 + r as f64 * (std::f64::consts::PI * 3. / 4.).cos(),
        circle_y as f64 + r as f64 * (std::f64::consts::PI * 3. / 4.).sin()
    );

    let (p3x, p3y) = (
        circle_x as f64 + r as f64 * (std::f64::consts::PI * 5. / 4.).cos(),
        circle_y as f64 + r as f64 * (std::f64::consts::PI * 5. / 4.).sin()
    );

    let (p4x, p4y) = (
        circle_x as f64 + r as f64 * (std::f64::consts::PI * 7. / 4.).cos(),
        circle_y as f64 + r as f64 * (std::f64::consts::PI * 7. / 4.).sin()
    );

    let thickness = (r * 0.11) as u8;

    // Draw the first line of the white 'X'
    draw_image_line(p1x as u32, p1y as u32, p3x as u32, p3y as u32, thickness, image, WHITE);

    // Draw the second line of the 'X'
    draw_image_line(p2x as u32, p2y as u32, p4x as u32, p4y as u32, thickness, image, WHITE);
    for w in (circle_x - r) as i32..(circle_x + r) as i32 {
        for h in (circle_y - r) as i32..(circle_y + r) as i32 {
            let dx = circle_x - w as f32;
            let dy = circle_y - h as f32;
            let d = ((dx * dx) + (dy * dy)).sqrt();

            if d < r && d > r - (r * 0.09){
                image.set_pixel(w as u32, h as u32, BLACK); 
            }
        } 
    }
}
