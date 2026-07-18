# surrealism-password

Secure password hashing and verification for Surrealism, using Argon2.

```surql
DEFINE MODULE mod::password AS f"bucket:/password.surli";

LET $hash = mod::password::hash("hunter2");
RETURN mod::password::verify("hunter2", $hash); -- true
```

| Function | Signature | Description |
|---|---|---|
| `hash` | `(password: string) -> string` | Hashes a password with Argon2, returning a PHC-format string |
| `verify` | `(password: string, hash: string) -> bool` | Verifies a password against a stored PHC hash string |
