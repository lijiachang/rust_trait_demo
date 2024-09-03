/// 存储具有共同trait的结构体
use super::traits;
use traits::PatientRole;

pub struct PatientList {
    pub patients: Vec<Box<dyn PatientRole>>
    // 需要将结构体包装到一个box中，因为我们不知道编译时的大小。不同大小的不同结构体可以实现相同的trait。
    // Box是堆内存上的指针。
    // 我们知道指针的大小，所以知道在编译时添加到向量的内存大小。
}