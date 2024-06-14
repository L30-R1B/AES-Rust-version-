
pub fn matrix_init(vetor: &[u8]) -> [[u8; 4]; 4] {
    [
        [vetor[0], vetor[1], vetor[2], vetor[3]],
        [vetor[4], vetor[5], vetor[6], vetor[7]],
        [vetor[8], vetor[9], vetor[10], vetor[11]],
        [vetor[12], vetor[13], vetor[14], vetor[15]],
    ]
}
