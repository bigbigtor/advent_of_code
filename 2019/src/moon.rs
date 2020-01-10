use itertools::Itertools;

#[derive(Debug)]
pub struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    pub fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            pos: [x, y, z],
            vel: [0; 3],
        }
    }

    pub fn apply_gravity(moons: &mut Vec<Moon>) {
        for pair in (0..moons.len()).into_iter().combinations(2) {
            let a = pair[0];
            let b = pair[1];
            (0..3).for_each(|i| {
                let val = moons[a].pos[i].cmp(&moons[b].pos[i]) as i64;
                moons[a].vel[i] -= val;
                moons[b].vel[i] += val;
            });
        }
    }

    pub fn apply_velocity(moons: &mut Vec<Moon>) {
        moons.iter_mut()
             .for_each(|moon| {
                (0..3).for_each(|i| moon.pos[i] += moon.vel[i]);
             })
    }

    pub fn get_system_total_energy(moons: &Vec<Moon>) -> u64 {
        moons.iter()
             .map(|m| m.get_total_energy())
             .sum()
    }

    fn get_total_energy(&self) -> u64 {
        self.get_potential_energy() *
            self.get_kinetic_energy()
    }

    fn get_potential_energy(&self) -> u64 {
        self.pos.iter()
                .map(|&p| if p < 0 { -p } else { p } as u64)
                .sum()
    }

    fn get_kinetic_energy(&self) -> u64 {
        self.vel.iter()
                .map(|&v| if v < 0 { -v } else { v } as u64)
                .sum()
    }

    pub fn get_cycle_steps(moons: &Vec<Moon>) -> u64 {
        let x_steps = Moon::get_cycle_steps_axis(&moons, 0);
        let y_steps = Moon::get_cycle_steps_axis(&moons, 1);
        let z_steps = Moon::get_cycle_steps_axis(&moons, 2);
        Moon::lcm(x_steps, Moon::lcm(y_steps, z_steps))
    }

    pub fn get_cycle_steps_axis(moons: &Vec<Moon>, axis: usize) -> u64 {
        let mut result = 1;
        let ipos: Vec<i64> = moons.iter()
                                  .map(|m| m.pos[axis])
                                  .collect();
        let ivel: Vec<i64> = moons.iter()
                                  .map(|m| m.vel[axis])
                                  .collect();
        let mut pos = ipos.clone();
        let mut vel = ivel.clone();
        loop {
            for pair in (0..pos.len()).into_iter().combinations(2) {
                let (a, b) = (pair[0], pair[1]);
                let val = pos[a].cmp(&pos[b]) as i64;
                vel[a] -= val;
                vel[b] += val;
            }
            pos.iter_mut().enumerate().for_each(|(i, p)| *p += vel[i]);
            if ipos == pos && ivel == vel {
                break;
            } else {
                result += 1;
            }
        }
        result
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a * b / Moon::gcd(a, b)
    }

    fn gcd(x: u64, y: u64) -> u64 {
        let mut x = x;
        let mut y = y;
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_moons() -> Vec<Moon> {
        let mut moons = Vec::new();
        moons.push(Moon::new(-1, 0, 2));
        moons.push(Moon::new(2, -10, -7));
        moons.push(Moon::new(4, -8, 8));
        moons.push(Moon::new(3, 5, -1));
        moons
    }

    #[test]
    fn step1() {
        let mut moons = build_moons();
        for _time in 0..1 {
            Moon::apply_gravity(&mut moons);
            Moon::apply_velocity(&mut moons);
        }
        assert_eq!(moons[0].pos, [2, -1, 1]);
        assert_eq!(moons[1].pos, [3, -7, -4]);
        assert_eq!(moons[2].pos, [1, -7, 5]);
        assert_eq!(moons[3].pos, [2, 2, 0]);

        assert_eq!(moons[0].vel, [3, -1, -1]);
        assert_eq!(moons[1].vel, [1, 3, 3]);
        assert_eq!(moons[2].vel, [-3, 1, -3]);
        assert_eq!(moons[3].vel, [-1, -3, 1]);
    }

}
