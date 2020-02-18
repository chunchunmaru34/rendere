use std::fs;

pub struct Model {
    pub vertexes: Vec<[f32; 3]>,
    pub faces: Vec<[u32; 3]>,
}

impl Model {
    pub fn load(path: &str) -> Self {
        let file = fs::read_to_string(path).unwrap();
        let iter = file.split('\n');

        let mut vertexes: Vec<[f32; 3]> = Vec::new();
        let mut faces: Vec<[u32; 3]> = Vec::new();

        for slice in iter {
            let chunk: Vec<&str> = slice.trim().split(' ').collect();

            if chunk.len() != 4 {
                continue;
            }

            let line_type = chunk.get(0).unwrap();

            match line_type {
                &"f" => {
                    let face_values = chunk.get(1..).unwrap();
                    let f1: &str = face_values[0]
                        .split('/')
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap();
                    let f2: &str = face_values[1]
                        .split('/')
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap();
                    let f3: &str = face_values[2]
                        .split('/')
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap();

                    faces.push([
                        f1.parse::<u32>().unwrap() - 1,
                        f2.parse::<u32>().unwrap() - 1,
                        f3.parse::<u32>().unwrap() - 1,
                    ]);
                }
                &"v" => vertexes.push([
                    chunk.get(1).unwrap().parse::<f32>().unwrap(),
                    chunk.get(2).unwrap().parse::<f32>().unwrap(),
                    chunk.get(3).unwrap().parse::<f32>().unwrap(),
                ]),
                _ => {}
            }
        }

        Self { vertexes, faces }
    }
}
