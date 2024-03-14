pub type ForceVector = (f32, f32);
use super::camera::*;

pub trait PhysObj {
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
    /*
    for i in 0..bodies.len() {
        let (fx, fy) = calculate_gravity(bodies[i], ship);
        let theta = f32:: atan2(fy, fx);
        bodies[i].add_vector((fx, fy));
        ship.add_vector((-fx, -fy))
    }
    */
}
pub fn check_collisions(stars: &mut Vec<Box<dyn PhysObj>>) {
    for i in 0..stars.len() {
        for j in i+1..stars.len() {
            let dx = stars[j].xpos() - stars[i].xpos();
            let dy = stars[j].ypos() - stars[i].ypos();

            let distance = (dx*dx + dy*dy).sqrt();

            let nx = dx / distance;
            let ny = dy / distance;
            
            let relx = (stars[i].xvel() - stars[j].xvel()).abs();
            let rely = (stars[i].yvel() - stars[j].yvel()).abs();
            if distance <= (stars[i].radius() + stars[j].radius() + 2.) {
                if distance < (stars[i].radius() + stars[j].radius() + 2.) {
                    // Calculate the overlap between the two circles (how much one circle
                    // has penetrated into the other)
                    let overlap = distance - (distance - stars[i].radius() + distance - stars[j].radius());

                    // Displace the current circle along the normal by half of the overlap
                    stars[i].update_xpos(-(overlap * nx / distance));
                    stars[i].update_ypos(-(overlap * ny / distance));

                    // Displace the other circle along the normal by half of the overlap
                    stars[j].update_xpos((overlap * nx / distance));
                    stars[j].update_ypos((overlap * ny / distance));
                }

                let dx = stars[j].xpos() - stars[i].xpos();
                let dy = stars[j].ypos() - stars[i].ypos();

                let distance = (dx*dx + dy*dy).sqrt();

                let nx = dx / distance;
                let ny = dy / distance;
 
                // Calculate the relative velocity
                let rvx = stars[j].xvel() - stars[i].xvel();
                let rvy = stars[j].yvel() - stars[i].yvel();

                // Calculate the relative velocity in terms of the normal direction
                let norm_vec = rvx * nx + rvy * ny;

                // Do not resolve if velocities are separating
                if norm_vec > 0. {
                    continue;
                }

                // Calculate the impulse scalar
                let e = 1.;  // Coefficient of restitution
                let impulse = -(1. + e) * norm_vec / ((1. / stars[i].mass() as f32) +  (1. / stars[j].mass() as f32));

                // Apply impulse
                let impulse_x = impulse * nx;
                let impulse_y = impulse * ny;
                let imass = stars[i].mass();
                let jmass = stars[j].mass();
                stars[i].update_xvel(-(1. / imass as f32 * impulse_x));
                stars[i].update_yvel(-(1. / imass as f32 * impulse_y));
                stars[j].update_xvel((1. / jmass as f32 * impulse_x));
                stars[j].update_yvel((1. / jmass as f32 * impulse_y));
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

