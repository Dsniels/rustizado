
pub trait Colliadable<T>{
    fn collide(&self, other: &T)->bool;
    fn collides(&self, others: &[T])->bool{
        for other in others {
            if self.collide(other){
                return true;
            }
        }
        return false;
    }
}