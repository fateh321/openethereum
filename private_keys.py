import binascii
from web3.auto import w3
with open("/home/oe/parity1/keys/PoA/UTC--2022-05-24T19-19-34Z--42014e54-fe9e-83ae-7692-ada94f39e87f") as keyfile:
    encrypted_key = keyfile.read()
    private_key = w3.eth.account.decrypt(encrypted_key, 
                                             'node1')

private_key = binascii.b2a_hex(private_key)

print(private_key)
