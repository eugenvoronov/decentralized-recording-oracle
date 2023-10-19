# Decentralized Recording Oracle

This project builds on the concept of decentralization in the context of a Recording Oracle. The current repository is a work in progress and provides a fundamental implementation of a Recording Oracle using Intel Software Guard Extensions (SGX) enclaves.

The project encompasses both the Recording Oracle Master and the Recording Oracle Worker implementations, each playing a vital role in ensuring secure and trusted operations within an SGX enclave.

## **Key Features of SGX Enclaves**

- SGX enclaves establish secure computing areas within the CPU, safeguarding code and data from external threats.
- Enclaves provide isolation, data confidentiality, and the ability to securely create and destroy enclaves as needed.
- SGX ensures code integrity and protection against external tampering, even in compromised environments.
- Enclave reports serve as remote verification, confirming the secure execution of code on remote devices.

## **Workflow Overview**

**1. Select Oracle:** The Recording Oracle Master generates a random number within the enclave.

**2. Choose Worker:** Using the random number, the Master selects a Recording Oracle Worker from a pool.

**3. Secure Execution:** The Worker securely executes tasks within the enclave, isolating code and data from external interference.

**4. IPFS Upload:** Upon task completion, the Worker uploads results to IPFS.

**5. Enclave Report:** An enclave report is generated during execution, providing proof of secure operation. The Master verifies its integrity.

**6. Zero-Knowledge Proof:** The Master prepares a zero-knowledge proof based on the enclave report, ensuring fair selection and data integrity.

**7. Blockchain Storage:** Data is stored in the blockchain, maintaining transparency and immutability.


## **Makefile Commands:**
- make (will compile everything)
- make host (will only compile the host part)
- make enclave (will only compile the enclave part)
- make clean (will clean the objects/C edl files generated)
- make clean_host (will clean the objects/C edl files generated for the host only)
- make clean_enclave (will clean the objects/C edl files generated for the enclave only)
- make fclean (will clean objects/C edl files and the binaries, plus calling cargo clean for everything)
- make fclean_host (will clean objects/C edl files and the binaries, plus calling cargo clean for the host only)
- make fclean_enclave (will clean objects/C edl files and the binaries, plus calling cargo clean for the enclave only)
- make re (re as relink, will clean everything then compile everything again)
- make re_host (re as relink, will clean the host part then compile it again)
- make re_enclave (re as relink, will clean the enclave part then compile it again)
