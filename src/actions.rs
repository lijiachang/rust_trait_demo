// 为了执行诸如 更新数据库 或 将数据发送到服务器之类的操作，可以将人员结构体传递给函数
use super::traits;
use traits::{ClinicalSkills, AdvanceMedical, PatientRole};

/// 收治病人。（传入的临床职业者clinician，是任何具有ClinicalSkills trait的人员，即所有的临床职业者结构体）
pub fn admin_patient<Y: ClinicalSkills>(patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is being admitted", patient.get_name())
}

/// 诊断患者。（具有AdvancedMedical trait才能进行诊断）
pub fn diagnose_patient<Y: AdvanceMedical>(patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is being diagnosed", patient.get_name())
}

/// 开处方 (检查是否能can_prescribe）
pub fn prescribe_meds<Y: ClinicalSkills>(patient: &Box<dyn PatientRole>, clinician: &Y) {
    if clinician.can_prescribe() {
        println!("{} is being prescribe medication", patient.get_name())
    } else {
        panic!("clinician cannot prescribe medication")
    }
}

/// 给病人用药
pub fn administer_meds<Y: ClinicalSkills>(patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is having meds administered", patient.get_name())
}

/// 让病人出院
pub fn discharge_patient<Y: ClinicalSkills>(patient: &Box<dyn PatientRole>, _clinician: &Y) {
    println!("{} is being discharge", patient.get_name())
}