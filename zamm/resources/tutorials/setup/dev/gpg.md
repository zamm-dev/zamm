# Creating a GPG key

Following the instructions [here](https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key), create a new GPG key as such:

```bash
$ gpg --full-generate-key
gpg (GnuPG) 2.2.27; Copyright (C) 2021 Free Software Foundation, Inc.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Please select what kind of key you want:
   (1) RSA and RSA (default)
   (2) DSA and Elgamal
   (3) DSA (sign only)
   (4) RSA (sign only)
  (14) Existing key from card
Your selection? 1
RSA keys may be between 1024 and 4096 bits long.
What keysize do you want? (3072) 4096
Requested keysize is 4096 bits
Please specify how long the key should be valid.
         0 = key does not expire
      <n>  = key expires in n days
      <n>w = key expires in n weeks
      <n>m = key expires in n months
      <n>y = key expires in n years
Key is valid for? (0) 0
Key does not expire at all
Is this correct? (y/N) y

You need a user ID to identify your key; the software constructs the user ID
from the Real Name, Comment and Email Address in this form:
    "Heinrich Heine (Der Dichter) <heinrichh@duesseldorf.de>"

```

You'll want to provide the name and email separately, as if you provide it all at the same time, you get:

```
Real name: John Smith <john.smith@anon.org>
Invalid character in name
The characters '<' and '>' may not appear in name

```

Instead, do

```
Real name: John Smith
E-mail address: john.smith@anon.org
Comment: 
You selected this USER-ID:
    "John Smith <john.smith@anon.org>"

```

Check that it's been created with

```bash
$ gpg --list-secret-keys
/home/amos/.gnupg/pubring.kbx
-----------------------------
sec   rsa4096 2023-11-14 [SC]
      AAAAXXXX
uid           [ultimate] John Smith <john.smith@anon.org>
ssb   rsa4096 2023-11-14 [E]

```

Encrypt something with it:

```bash
$ echo "hello secret" > test.txt
$ gpg --encrypt --recipient john.smith@anon.org --armor test.txt
$ cat test.txt.asc            
-----BEGIN PGP MESSAGE-----

GGGGXXXX
-----END PGP MESSAGE-----
```

Back it up in ASCII format with

```bash
$ gpg --armor --export-secret-keys --output backup.gpg
$ cat backup.gpg    
-----BEGIN PGP PRIVATE KEY BLOCK-----
...
-----END PGP PRIVATE KEY BLOCK-----

```

Delete it:

```bash
$ gpg --delete-secret-key AAAAXXXX
gpg (GnuPG) 2.2.27; Copyright (C) 2021 Free Software Foundation, Inc.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.


sec  rsa4096/BBBBXXXX 2023-11-14 John Smith <john.smith@anon.org>

Delete this key from the keyring? (y/N) y
This is a secret key! - really delete? (y/N) y
$ gpg --list-keys                                                 
/home/amos/.gnupg/pubring.kbx
-----------------------------
pub   rsa4096 2023-11-14 [SC]
      AAAAXXXX
uid           [ultimate] John Smith <john.smith@anon.org>
sub   rsa4096 2023-11-14 [E]
$ gpg --delete-key AAAAXXXX
gpg (GnuPG) 2.2.27; Copyright (C) 2021 Free Software Foundation, Inc.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.


pub  rsa4096/BBBBXXXX 2023-11-14 John Smith <john.smith@anon.org>

Delete this key from the keyring? (y/N) y

```

Verify it's gone:

```bash
$ gpg --list-secret-keys
```

Import it back in:

```bash
$ gpg --import backup.gpg                             
gpg: key D7EE6B5BAD9A9E3E: public key "John Smith <john.smith@anon.org>" imported
gpg: key D7EE6B5BAD9A9E3E: secret key imported
gpg: Total number processed: 1
gpg:               imported: 1
gpg:       secret keys read: 1
gpg:   secret keys imported: 1

```

Decrypt the original message:

```bash
$ gpg --output unencrypted.txt --decrypt test.txt.asc
gpg: encrypted with 4096-bit RSA key, ID AAAAXXXX, created 2023-11-14
      "John Smith <john.smith@anon.org>"
$ cat unencrypted.txt 
hello secret

```
