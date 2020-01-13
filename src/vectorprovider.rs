use crate::iprovider::Provider;


//包装对象不带T，可以设计一个泛型的layer，待定
pub struct VectorLayer {}

impl Provider<T> For VectorLayer{
    fn getbyextent(env:Rect<f64>)->Vec<T>{
        vec![0.0f64,0.0f32]
    }
}