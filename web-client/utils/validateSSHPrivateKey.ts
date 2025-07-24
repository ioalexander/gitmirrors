type ValidationResult = { result: boolean; message: string };

export function validateSSHPrivateKey(input: string): ValidationResult {
  if (typeof input !== "string" || input.trim() === "") {
    return { result: false, message: "Input must be a non-empty string!" };
  }

  // Normalize line endings
  const key = input.trim().replace(/\r\n/g, "\n");

  // Regex to match header, footer, and capture base64 body
  const regex =
    /^-----BEGIN (OPENSSH|RSA|DSA|EC|ED25519) PRIVATE KEY-----\n([\sA-Za-z0-9+/=]+)\n-----END \1 PRIVATE KEY-----$/m;

  const match = key.match(regex);
  if (!match) {
    return {
      result: false,
      message:
        "Key must start and end with matching PEM headers and footers of supported types!",
    };
  }

  // Extract and clean base64 body
  const base64Body = match[2].replace(/\s+/g, "");

  if (base64Body.length === 0) {
    return { result: false, message: "Key body is empty!" };
  }

  // Base64 validation: only A-Z,a-z,0-9,+,/ and padding =
  if (
    !/^[A-Za-z0-9+/]+={0,2}$/.test(base64Body) ||
    base64Body.length % 4 !== 0
  ) {
    return { result: false, message: "Key body is not valid Base64!" };
  }

  // Heuristic length check (adjust as needed)
  if (base64Body.length < 300) {
    return { result: false, message: "Key appears too short to be valid!" };
  }

  return { result: true, message: "Valid SSH private key." };
}
