use stray::prelude::*;
use legion::*;


#[system(for_each)]
fn draw(draw: &mut Canvas){
    let vertices = vec![
        Vertex::new(-500, -500,0), Vertex::new(500, -500,0), Vertex::new(-500, 500,0), 
    ];
    let color = StrayColor{
        r: 255,
        g: 255,
        b: 100,
        a: 1.0,
    };
    let material = StandardMaterial::new(color);
    draw.set_vertices(vertices);
    draw.set_material(material);
}

fn main(){
    Stray::new()
        .with_title("Stray App")
        .push((
            Canvas::init(0),
            Transform2D::ZERO
        ))
        .add_system(draw_system())
        .build()
        .run();
}