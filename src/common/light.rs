use crate::math::vector::{Vector3f, Vector4f};

pub struct Light {
    position: Vector3f,
    intensity: Vector3f
}

impl Light {
    pub fn new(p: Vector3f, i: Vector3f) -> Light {
        Light {
            position: p,
            intensity: i
        }
    } 
}

pub fn compute_light(pos: &Vector3f, normal: &Vector3f, lights: &Vec<Light>, 
    ka: &Vector3f, kd: &Vector3f, ks: &Vector3f, 
    eye_pos: &Vector3f, 
) -> Vector3f {

    let p = 150;
    let mut r = Vector3f::new();
    let view_dir = eye_pos.sub(pos);
    let amb_light_intensity = Vector3f::new_3(10.0, 10.0, 10.0);

    for l in lights {
        let distance = l.position.sub(pos).length();
        let mut diffuse = Vector3f::new();
        let mut specular = Vector3f::new();
        let mut ambient = Vector3f::new();

        let mut light_dir = l.position.sub(pos);
        light_dir.normlize();
        // println!("{:?}", light_dir);
        let mut half = view_dir.add(&light_dir);
        half.normlize();
        for i in 0..3 {
            let intensity = l.intensity.v[i]/(distance * distance);

            diffuse.v[i] = kd.v[i] * intensity * normal.dot_product(&light_dir).max(0.0);
            specular.v[i] = ks.v[i] * intensity * normal.dot_product(&half).max(0.0).powi(p);
            ambient.v[i] = amb_light_intensity.v[i] * ka.v[i];

            r.v[i] += diffuse.v[i];
            r.v[i] += specular.v[i];
            r.v[i] += ambient.v[i];
        }
    }

    r
}