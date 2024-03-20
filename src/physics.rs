pub type ForceVector = (f32, f32);

use std::any::Any;
use rayon::prelude::*;
use super::camera::*;
use super::star::*;

pub trait PhysObj {
    
    fn as_any(&self) -> &dyn Any;

    fn xpos(&self) -> f32;
    fn ypos(&self) -> f32;
    fn xvel(&self) -> f32;
    fn yvel(&self) -> f32;
    fn mass(&self) -> u64;
    fn radius(&self) -> f32;
    fn force_vectors(&self) -> Vec<ForceVector>;
    fn update(&mut self);
    fn update_xvel(&mut self, update_val: f32);
    fn update_yvel(&mut self, update_val: f32);
    fn add_vector(&mut self, force_vec: ForceVector);
    fn draw(&mut self, camera: &mut ZCamera);
    fn update_xpos(&mut self, update_val: f32);
    fn update_ypos(&mut self, update_val: f32);
}
/*
pub fn calculate_gravity<T: PhysObj>(body1: &T, body2: &T) -> ForceVector {
    //TODO i wonder if, in this fn somewhere, we can
    //actually check for collisions too.
    let (x0, y0) = (body1.xpos(), body1.ypos());
    let (x1, y1) = (body2.xpos(), body2.ypos());
    let (m0, m1) = (body1.mass() as f32, body2.mass() as f32);
    
    let dx = x1 - x0;
    let dy = y1 - y0;
    let theta = f32::atan2(dy, dx);
    let r = (dx*dx + dy*dy).sqrt();
    let g = 0.000000001;
    let f = g * (m0 * m1) / (r * r);


    // these are the x,y components of the
    // force of gravity that body1 feels from 
    // body2 ( i hope )
    
    let fx = f * f32::cos(theta);
    let fy = f * f32::sin(theta);

    (fx, fy)
    // therefore... i think that the body2
    // components are just the negative 
    // components of body1 ?
    // ( im picturing just the opposite vec-
    // tor in my head )

}
*/
pub fn calculate_gravity(body1: &dyn PhysObj, body2: &dyn PhysObj) -> ForceVector {
    let (x0, y0) = (body1.xpos(), body1.ypos());
    let (x1, y1) = (body2.xpos(), body2.ypos());
    let (m0, m1) = (body1.mass() as f32, body2.mass() as f32);
    
    let dx = x1 - x0;
    let dy = y1 - y0;
    let theta = f32::atan2(dy, dx);
    let r2 = (dx*dx + dy*dy);
    let g = 0.000000001;
    let f = g * (m0 * m1) / r2;


    // these are the x,y components of the
    // force of gravity that body1 feels from 
    // body2 ( i hope )
    
    let fx = f * f32::cos(theta);
    let fy = f * f32::sin(theta);

    (fx, fy)
    // therefore... i think that the body2
    // components are just the negative 
    // components of body1 ?
    // ( im picturing just the opposite vec-
    // tor in my head )

}
pub fn update_gravity_physics(
    bodies: &mut Vec<Box<dyn PhysObj>>,
//    ship: &mut dyn PhysObj
) {
     // Calculate all the forces
    for i in 0..bodies.len() {
        for j in i+1..bodies.len() {
            let (fx, fy) = calculate_gravity(&*bodies[i], &*bodies[j]);
            let theta = f32::atan2(fy, fx);
            bodies[i].add_vector((fx, fy));
            bodies[j].add_vector((-fx, -fy));
        }
    }
}

/*
pub fn update_gravity_physics(
    bodies: &mut Vec<Box<dyn PhysObj>>,
) {
   bodies.par_iter_mut().enumerate().for_each(|(i, body_i)| {
        for j in i+1..bodies.len() {
            let (fx, fy) = calculate_gravity(&*body_i, &*bodies[j]);
            let theta = f32::atan2(fy, fx);
            body_i.add_vector((fx, fy));
            bodies[j].add_vector((-fx, -fy));
        }
    });
}

*/
pub fn check_collisions(bodies: &mut Vec<Box<dyn PhysObj>>) {
    for i in 0..bodies.len() {
        for j in i+1..bodies.len() {
            let dx = bodies[j].xpos() - bodies[i].xpos();
            let dy = bodies[j].ypos() - bodies[i].ypos();

            let distance = (dx*dx + dy*dy).sqrt();

            let nx = dx / distance;
            let ny = dy / distance;
            
            let relx = (bodies[i].xvel() - bodies[j].xvel()).abs();
            let rely = (bodies[i].yvel() - bodies[j].yvel()).abs();
            if distance <= (bodies[i].radius() + bodies[j].radius() + 2.) {

                let potential_star1: Option<&Star> = bodies[i].as_any().downcast_ref::<Star>();
                if let Some(star1) = potential_star1 {
                    let potential_star2: Option<&Star> = bodies[j].as_any().downcast_ref::<Star>();
                    if let Some(star2) = potential_star2 {
                        //println!("Collsion between two stars dectected!");
                    }
                }


                if distance < (bodies[i].radius() + bodies[j].radius() + 2.) {
                    // Calculate the overlap between the two circles (how much one circle
                    // has penetrated into the other)
                    let overlap = distance - (distance - bodies[i].radius() + distance - bodies[j].radius());

                    // Displace the current circle along the normal by half of the overlap
                    bodies[i].update_xpos(-(overlap * nx / distance));
                    bodies[i].update_ypos(-(overlap * ny / distance));

                    // Displace the other circle along the normal by half of the overlap
                    bodies[j].update_xpos((overlap * nx / distance));
                    bodies[j].update_ypos((overlap * ny / distance));
                }

                let dx = bodies[j].xpos() - bodies[i].xpos();
                let dy = bodies[j].ypos() - bodies[i].ypos();

                let distance = (dx*dx + dy*dy).sqrt();

                let nx = dx / distance;
                let ny = dy / distance;
 
                // Calculate the relative velocity
                let rvx = bodies[j].xvel() - bodies[i].xvel();
                let rvy = bodies[j].yvel() - bodies[i].yvel();
                let norm_vec = rvx * nx + rvy * ny;

                // Do not resolve if velocities are separating
                if norm_vec > 0. {
                    continue;
                }

                // Calculate the impulse scalar
                let e = 1.;  // Coefficient of restitution
                let impulse = -(1. + e) * norm_vec / ((1. / bodies[i].mass() as f32) +  (1. / bodies[j].mass() as f32));

                // Apply impulse
                let impulse_x = impulse * nx;
                let impulse_y = impulse * ny;
                let imass = bodies[i].mass();
                let jmass = bodies[j].mass();
                bodies[i].update_xvel(-(1. / imass as f32 * impulse_x));
                bodies[i].update_yvel(-(1. / imass as f32 * impulse_y));
                bodies[j].update_xvel((1. / jmass as f32 * impulse_x));
                bodies[j].update_yvel((1. / jmass as f32 * impulse_y));
            }
        }
    }
}


/*
pub fn update_gravity_physics<T: PhysObj>(bodies: &mut [T], ship: &mut T) {
    // Calculate all the forces
    for i in 0..bodies.len() {
        for j in i+1..bodies.len() {
            let (fx, fy) = calculate_gravity(&bodies[i], &bodies[j]);
            let theta = f32::atan2(fy, fx);
            bodies[i].add_vector((fx, fy));
            bodies[j].add_vector((-fx, -fy));
        }
    }
    for i in 0..bodies.len() {
        let (fx, fy) = calculate_gravity(&bodies[i], ship);
        let theta = f32:: atan2(fy, fx);
        bodies[i].add_vector((fx, fy));
        ship.add_vector((-fx, -fy))
    }
}
*/

fn fast_inverse_sqrt(n: f32) -> f32 {
    let i = unsafe { std::mem::transmute::<f32, i32>(n) };
    let j = 0x5f3759df - (i >> 1);
    let y = unsafe { std::mem::transmute::<i32, f32>(j) };
    y * (1.5f32 - 0.5f32 * n * y * y)
}

fn fast_root(n :f32) -> f32 {
    1. / fast_inverse_sqrt(n)
}

