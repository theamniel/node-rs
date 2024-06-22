import test from 'ava';
import { encrypt, decrypt } from '../../packages/security/lib';

// For test
const
  // Secret key (don't use this for production)
  secret = Buffer.from('50b306acfcb1c84c51c68a9df4db221f49a95a74c6d4d5933b9eac70e63e7dda', 'hex'), // 32
  // Inicialization vector (iv, don't use this for production)
  nonce = Buffer.from('fad7a4c8600e35cebdbd095d36b7de0c', 'hex'), // 16
  // Data to encrypt
  text = 'Hello, world!',
  // Cipher text (data encrypted in a hex string)
  ciphertext = '2f00e70900457dd7402920556c';

test('encrypt', (t) => {
  const encrypted = encrypt(text, secret, nonce);
  t.true(!!encrypted);
  t.is(encrypted, ciphertext, 'Invalid encrypted data');
});

test('decrypt', (t) => {
  const decrypted = decrypt(ciphertext, secret, nonce);
  t.true(!!decrypted);
  t.is(decrypted, text, 'Invalid decrypted data');
});
