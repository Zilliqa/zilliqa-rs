pub mod error;
pub mod util;
// export const normalizePrivateKey = (privateKey: string): string => {
//     try {
//       if (!validation.isPrivateKey(privateKey)) {
//         throw new Error("Private key is not correct");
//       }
//       const normalized = privateKey.toLowerCase().replace("0x", "");
//       if (!verifyPrivateKey(normalized)) {
//         throw new Error("Private key is not correct");
//       }
//       return normalized;
//     } catch (error) {
//       throw error;
//     }
//   };
