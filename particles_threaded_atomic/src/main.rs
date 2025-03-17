#![allow(dead_code)]
use std::time::Instant;
use std::sync::Arc;
//use std::rc::Rc;

const NUM_OF_THREADS: usize = 5;
const NUM_OF_PARTICLES: usize = 10000;
const SIZE_OF_CONTAINER: f32 = 10.0;
const PARTICLES_PER_THREAD: usize = NUM_OF_PARTICLES / NUM_OF_THREADS;

#[derive(Debug, Copy, Clone)]
struct Particle
{
    x: f32,
    y: f32,
}
impl Particle
{
    fn new(x: f32, y: f32) -> Self
    {
        return Self{x, y};
    }

    fn move_by(&mut self, dx: f32, dy: f32)
    {
        self.x += dx;
        self.y += dy;

        if self.x < 0.0 { self.x = 0.0};
        if self.y < 0.0 { self.y = 0.0};

        if self.x > SIZE_OF_CONTAINER { self.x = SIZE_OF_CONTAINER};
        if self.y > SIZE_OF_CONTAINER { self.y = SIZE_OF_CONTAINER};
        
    }

    fn collide(&self, p2: &Particle) -> bool
    {
        let dx: f32 = self.x - p2.x;
        let dy: f32 = self.y - p2.y;

        let distance = ((dx * dx) + (dy * dy)).sqrt();
        if distance < 0.1
        {
            return true;
        }
        return false;

    }
}

struct ParticleSystem
{
    // moved: bool,
    particles: Vec<Particle>,
}
impl ParticleSystem
{

    fn new() -> Self
    {
        let mut particles_vec = Vec::new();
        for _i in 0..NUM_OF_PARTICLES
        {
            let new_particle = Particle::new(rand::random::<f32>() * 10.0, rand::random::<f32>() * 10.0);
            particles_vec.push(new_particle);
        }
        return Self{particles: particles_vec};
        
    
    }
    
    fn move_particles(&mut self)
    {
        for i in 0..NUM_OF_PARTICLES
        {
            self.particles[i].move_by(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
        }
    }

    fn check_collisions()
    {

    }

    fn move_loop_10s(&mut self)
    {
        let start = Instant::now();
        for _i in 0..200000
        {
            self.move_particles();
        }
        println!("Time elapsed {:?}", start.elapsed());
    }
}

// fn thread_main (list: &mut [Particle])
// {
    
//     let start = Instant::now();
//     for _t in 0..200000
//     {
//         for i in &mut *list
//         {   
//             i.move_by(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
//         }
//     }
//     println!("Time elapsed for thread: {:?}", start.elapsed());
    
// }

// new thread main
fn thread_main (list: &mut [Particle])
{
    for i in &mut *list
    {   
        i.move_by(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
    }
    
}



// move particles for one step
// copy particles list for a read only to share between the collisions
// 




fn check_collisions(list: &[Particle], whole_list: &Vec<Particle>) -> i32
{
    let mut collisions: i32 = 0;
    for p1 in list
    {
        for p2 in whole_list
        {
            // prevent colliding with itself
            if p1.collide(p2)
            {
                collisions += 1;
            }
        }
    }

    // remove collisons with self which is just the number of paricles in list
    collisions -= list.len() as i32;

    println!("{} collisions in thread", collisions);
    return collisions;
}

fn main()
{
    let start = Instant::now();
    let mut particle_system = ParticleSystem::new();
    
    println!("x:{}, y:{}", particle_system.particles[0].x, particle_system.particles[0].y);
    //particle_system.move_loop_10s();
    
    let mut pool = scoped_threadpool::Pool::new(NUM_OF_THREADS as u32);

    for _i in 0..3
    {
    
        pool.scoped(|scope|
            {
                for slice in particle_system.particles.chunks_mut(PARTICLES_PER_THREAD)
                {
                    scope.execute(move || thread_main(slice));
                }
            });
            // join is automatic



        // could be a reference counter too?
        // just sharing the particles list between threads
        let whole_list = Arc::new(particle_system.particles.clone());

        // then check collisions:
        pool.scoped(|scope|
            {
                for slice in particle_system.particles.chunks(PARTICLES_PER_THREAD)
                {
                    let cloned_list = Arc::clone(&whole_list);
                    scope.execute(move || {check_collisions(slice, &cloned_list);});
                }
            });
            
        

    }

    println!("Simulation finished");
    println!("x:{}, y:{}", particle_system.particles[0].x, particle_system.particles[0].y);
    println!("Particles:{}, Threads:{}, Particles per thread:{}", NUM_OF_PARTICLES, NUM_OF_THREADS, PARTICLES_PER_THREAD);
    println!("Total time elapsed {:?}", start.elapsed());

}
