use core::arch::x86_64;
use core::mem::size_of;
use core::any::type_name;


fn main() {
    let a: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 12.0, 14.0, 16.0, 18.0, 10.0, 1.0];
    let b: [f64; 10] = [5.0, 6.0, 7.0, 8.0, 20.0, 22.0, 24.0, 26.0, 10.0, 2.0];
    
    let mut result: [f64; 10] = [0.0; 10];
    add_vectors_avx2_pd(&a, &b, &mut result);

    println!("{:?}", result);
    print_size_bytes::<x86_64::__m256d>();
    print_size_bytes::<f64>();

    permute();
}


fn print_size_bytes<T>() {
    let t_name = type_name::<T>();
    let t_size = size_of::<T>();
    println!("{:?} has {} byte(s)", t_name, t_size);
}

fn add_vectors_avx2_pd(a: &[f64], b: &[f64], result: &mut [f64]) {
    assert_eq!(a.len(), b.len(), "Input slices must have the same length");
    assert_eq!(a.len(), result.len(), "Input and output slices must have the same length");

    let n = a.len();

    let avx2_reg_size = size_of::<x86_64::__m256d>();
    let element_size = size_of::<f64>();
    let chunks = avx2_reg_size / element_size;

    println!("AVX2 register size: {} bytes", avx2_reg_size);
    println!("Element size: {} bytes", element_size);
    println!("Number of elements per AVX2 register: {}", chunks);

    unsafe {
        let mut i = 0;

        while i + chunks <= n {
            let vec_a = x86_64::_mm256_loadu_pd(a.as_ptr().add(i));
            let vec_b = x86_64::_mm256_loadu_pd(b.as_ptr().add(i));
            let vec_result = x86_64::_mm256_add_pd(vec_a, vec_b);
            x86_64::_mm256_storeu_pd(result.as_mut_ptr().add(i), vec_result);
            i += chunks;
        }

        while i < n {
            result[i] = a[i] + b[i];
            i += 1;
        }
    }
}

fn permute() {
    unsafe {
        let x: [f64; 4] = [1.0, 2.0, 3.0, 0.0];
        let mut result = [0f64; 8];

        let idx: x86_64::__m512i = x86_64::_mm512_set_epi64(1, 0, 2, 1, 0, 2, 1, 0); 
        let vx = x86_64::_mm256_load_pd(x.as_ptr());
        let vx_broadcasted = x86_64::_mm512_broadcast_f64x4(vx);

        let vx_permuted = x86_64::_mm512_permutexvar_pd(idx, vx_broadcasted);
        x86_64::_mm512_storeu_pd(result.as_mut_ptr(), vx_permuted);

        println!("{:?}", result);
    }
}
