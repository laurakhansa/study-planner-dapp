#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

// Struktur data yang akan menyimpan jadwal/materi belajar
#[contracttype]
#[derive(Clone, Debug)]
pub struct StudyPlan {
    pub id: u64,
    pub subject: String,
    pub topic: String,
}

#[contract]
pub struct StudyPlannerContract;

#[contractimpl]
impl StudyPlannerContract {
    // Fungsi untuk melihat jadwal belajar berdasarkan wallet address
    pub fn get_plans(env: Env, user: Address) -> Vec<StudyPlan> {
        // Ambil data dari storage berdasarkan Address pengguna
        env.storage().instance().get(&user).unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk membuat jadwal belajar baru (membutuhkan wallet)
    pub fn create_plan(env: Env, user: Address, subject: String, topic: String) -> String {
        // Autentikasi: Pastikan yang memanggil fungsi adalah pemilik wallet
        user.require_auth(); 

        // 1. Ambil data plan dari storage berdasarkan Address
        let mut plans: Vec<StudyPlan> = env.storage().instance().get(&user).unwrap_or(Vec::new(&env));
        
        // 2. Buat object StudyPlan baru
        let plan = StudyPlan {
            id: env.prng().gen::<u64>(),
            subject,
            topic,
        };
        
        // 3. Tambahkan plan baru ke list
        plans.push_back(plan);
        
        // 4. Simpan kembali ke storage dengan key Address pengguna
        env.storage().instance().set(&user, &plans);
        
        String::from_str(&env, "Study plan successfully added")
    }

    // Fungsi untuk menghapus jadwal berdasarkan ID (membutuhkan wallet)
    pub fn delete_plan(env: Env, user: Address, id: u64) -> String {
        // Autentikasi: Hanya pemilik wallet yang bisa menghapus datanya sendiri
        user.require_auth(); 

        // 1. Ambil data plan
        let mut plans: Vec<StudyPlan> = env.storage().instance().get(&user).unwrap_or(Vec::new(&env));

        // 2. Cari index plan yang akan dihapus
        for i in 0..plans.len() {
            if plans.get(i).unwrap().id == id {
                plans.remove(i);
                
                // 3. Simpan perubahan ke storage
                env.storage().instance().set(&user, &plans);
                return String::from_str(&env, "Study plan successfully deleted");
            }
        }

        String::from_str(&env, "Study plan not found")
    }
}

mod test;