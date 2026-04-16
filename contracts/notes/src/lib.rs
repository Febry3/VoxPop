#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// 1. Struktur data diubah dari Note menjadi Aspirasi
#[contracttype]
#[derive(Clone, Debug)]
pub struct Aspirasi {
    pub id: u64,
    pub text: String,
    pub timestamp: u64, // Tambahan fitur dari PRD: Timestamp Verification
}

// Storage key disesuaikan
const DATA_KEY: Symbol = symbol_short!("VOX_DATA");

#[contract]
pub struct VoxPopContract;

#[contractimpl]
impl VoxPopContract {
    // Fungsi The Lighthouse: Menampilkan daftar pesan publik
    pub fn get_wall(env: Env) -> Vec<Aspirasi> {
        // Ambil data aspirasi dari storage
        env.storage().instance().get(&DATA_KEY).unwrap_or(Vec::new(&env))
    }

    // Fungsi The Whisper: Untuk mengirim pesan rahasia
    pub fn post_whisper(env: Env, text: String) -> String {
        // Validasi panjang pesan (maksimal 280 karakter/byte sesuai PRD)
        if text.len() > 280 {
            return String::from_str(&env, "Gagal: Pesan lebih dari 280 karakter");
        }

        // 1. Ambil data aspirasi dari storage
        let mut wall: Vec<Aspirasi> = env.storage().instance().get(&DATA_KEY).unwrap_or(Vec::new(&env));
        
        // 2. Buat object aspirasi baru (Tanpa menyimpan identitas/Anonymous)
        let whisper = Aspirasi {
            id: env.prng().gen::<u64>(),
            text: text.clone(),
            timestamp: env.ledger().timestamp(), // Ambil waktu langsung dari jaringan Stellar
        };
        
        // 3. Tambahkan pesan baru ke Tembok
        wall.push_back(whisper);
        
        // 4. Simpan kembali ke storage
        env.storage().instance().set(&DATA_KEY, &wall);

        // 5. Fitur PRD (Event-Based Storage): Pancarkan event agar mudah dan murah dilacak oleh Frontend
        env.events().publish((symbol_short!("vox_pop"),), text);
        
        String::from_str(&env, "Aspirasi berhasil diabadikan")
    }

}

mod test;