# surrealism-hibp

Have I Been Pwned k-anonymity password-breach check for Surrealism.

```surql
DEFINE MODULE mod::hibp AS f"bucket:/hibp.surli";

RETURN mod::hibp::check_password("hunter2");
```

| Function | Signature | Description |
|---|---|---|
| `check_password` | `(password: string) -> int` | Number of times the password's hash appears in known breaches, or `0` |

Uses the [k-anonymity range API](https://haveibeenpwned.com/API/v3#PwnedPasswords):
only the first 5 characters of the SHA-1 hash of `password` are sent to the
API; the full hash never leaves the caller. No authentication is required.

Requires `allow_functions = ["http::get"]` and `api.pwnedpasswords.com` in
`allow_net`. Non-2xx responses surface as an `Err`.
