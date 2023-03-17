pub trait HiveObs {
    fn new(object: &str) -> Self;
    fn destroy(self) -> String;

}