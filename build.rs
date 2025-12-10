fn main() {
    println!("cargo:rerun-if-changed=.env");
    dotenvy::dotenv().ok();
    
    if let Ok(url) = std::env::var("FIREBASE_URL") {
        println!("cargo:rustc-env=FIREBASE_URL={}", url);
    } else {
        println!("cargo:warning=FIREBASE_URL not found in .env file!");
    }
}