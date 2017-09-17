#!/usr/bin/env python

import libs.alphabet
import json


def shift_cipher():
    """
    Generate a random flag and encrypt it with a random shift cipher.
    """
    ciphertext = ""
    shift = libs.alphabet.generate_random_shift()
    flag = libs.alphabet.generate_flag()

    # construct the ciphertext by shifting each char in the flag
    for char in flag:
        ciphertext += libs.alphabet.shift_char(char, shift)

    return {
        "statusCode": 200,
        "headers": {},
        "body": json.dumps({"flag": flag, "ciphertext": ciphertext, "shift": shift})
    }


def lambda_handler(event, context):
    return shift_cipher()
