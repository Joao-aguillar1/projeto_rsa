use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

// Função para calcular o máximo divisor comum
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

// Função para calcular o inverso modular
fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let mut mn = (m.clone(), a.clone());
    let mut xy = (Zero::zero(), One::one());

    while mn.1 != Zero::zero() {
        let q = &mn.0 / &mn.1;
        mn = (mn.1.clone(), &mn.0 - &q * &mn.1);
        xy = (xy.1.clone(), &xy.0 - &q * &xy.1);
    }

    if mn.0 != One::one() {
        None
    } else {
        Some((xy.0 % m + m) % m)
    }
}

// Função para verificar se um número é primo (teste simples)
fn is_prime(n: &BigInt) -> bool {
    if n <= &BigInt::from(1) {
        return false;
    }
    for i in 2..=1000 {
        if n % i == BigInt::from(0) && &BigInt::from(i) != n {
            return false;
        }
    }
    true
}

// Função para gerar um número primo aleatório
fn gen_prime(bits: u64) -> BigInt {
    let mut rng = thread_rng();
    loop {
        let candidate = rng.gen_bigint(bits);
        if candidate.sign() == num_bigint::Sign::Plus && is_prime(&candidate) {
            return candidate;
        }
    }
}

// Função para gerar chaves RSA
fn generate_keys(bits: u64) -> ((BigInt, BigInt), (BigInt, BigInt)) {
    let p = gen_prime(bits);
    let q = gen_prime(bits);
    let n = &p * &q;
    let phi = (&p - 1u32) * (&q - 1u32);

    let mut e = 65537.to_bigint().unwrap(); // valor comum para e
    while gcd(&e, &phi) != One::one() {
        e += 2;
    }

    let d = modinv(&e, &phi).expect("Não foi possível calcular o inverso modular");

    ((e.clone(), n.clone()), (d, n))
}

// Função para criptografar
fn encrypt(message: &BigInt, public_key: &(BigInt, BigInt)) -> BigInt {
    message.modpow(&public_key.0, &public_key.1)
}

// Função para descriptografar
fn decrypt(cipher: &BigInt, private_key: &(BigInt, BigInt)) -> BigInt {
    cipher.modpow(&private_key.0, &private_key.1)
}

fn main() {
    let bits = 64; // tamanho dos primos
    let (public_key, private_key) = generate_keys(bits);

    println!("Chave pública: {:?}", public_key);
    println!("Chave privada: {:?}", private_key);

    let message = 42.to_bigint().unwrap();
    let cipher = encrypt(&message, &public_key);
    let decrypted = decrypt(&cipher, &private_key);

    println!("Mensagem original: {}", message);
    println!("Mensagem criptografada: {}", cipher);
    println!("Mensagem descriptografada: {}", decrypted);
}
