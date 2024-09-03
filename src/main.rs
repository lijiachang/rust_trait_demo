mod actions;
mod objects;
mod people;
mod traits;

use people::{Patient, Nurse, Doctor};
use objects::PatientList;
use actions::{admin_patient, diagnose_patient, prescribe_meds, administer_meds, discharge_patient};

fn main() {
    // 定义当天的两名护士和医生
    let doctor = Doctor { name: String::from("MuYue") };
    let doctor_two = Doctor { name: String::from("ChenXi") };
    let nurse = Nurse { name: String::from("Rui") };
    let nurse_two = Nurse { name: String::from("Fen") };

    // 四名患者
    let patient_list = PatientList {
        patients: vec![
            Box::new(Patient { name: "pestilence".to_string() }),
            Box::new(Patient { name: "war".to_string() }),
            Box::new(Patient { name: "famine".to_string() }),
            Box::new(Patient { name: "death".to_string() }),
        ]
    };

    // 遍历患者，分配医生和护士照顾他们
    for patient in patient_list.patients {
        admin_patient(&patient, &nurse);
        diagnose_patient(&patient, &doctor);
        prescribe_meds(&patient, &doctor_two);
        administer_meds(&patient, &nurse_two);
        discharge_patient(&patient, &nurse);
    }
}
