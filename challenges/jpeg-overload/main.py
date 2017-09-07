#! /usr/bin/env python3
import os
import json
import random
import hashlib

import boto3

with open('cat.jpg', 'rb') as fp:
    CAT_FILE = fp.read()
with open('template.py', 'r') as fp:
    TEMPLATE = fp.read()
FLAG_ALPHABET = 'abcdefghijkmnopqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ23456789'

if os.getenv('AWS_EXECUTION_ENV') is not None:
    STORE_TO_S3 = True
    s3client = boto3.client('s3')
else:
    STORE_TO_S3 = False

S3_SETTINGS = {
    'ACL': 'public-read',
    'Bucket': os.getenv('S3_BUCKET', 'secret-ctf-challenges'),
    'ContentType': 'image/jpeg',
}
S3_KEY_BASE = os.getenv('S3_KEY_BASE', 'challenges/jpeg-overload/files/')

DEFAULT_FILE_PATH = '/tmp/cat.jpg'


def generate_flag():
    flag = 'flag_'
    for _ in range(0, 20):
        flag += random.choice(FLAG_ALPHABET)
    return flag


def encrypt_flag(flag, offset):
    flag = bytes(flag, 'ascii')
    return bytes(CAT_FILE[offset + i] ^ flag[i] for i in range(len(flag)))


def save_file(file_contents):
    md5 = hashlib.sha1(file_contents).hexdigest()
    filename = '%s.jpg' % md5
    if STORE_TO_S3:
        s3client.put_object(Key=os.path.join(S3_KEY_BASE, filename),
                            Body=file_contents,
                            **S3_SETTINGS)
    else:
        with open(DEFAULT_FILE_PATH, 'wb') as fp:
            fp.write(file_contents)

    return filename


def generate():
    offset = random.randint(0x256, 0x1aa00)
    flag = generate_flag()
    encrypted_flag = encrypt_flag(flag, offset)
    payload = TEMPLATE.format(offset=offset, cipher_flag=encrypted_flag)

    complete_file = CAT_FILE + payload.encode('ascii')
    filename = save_file(complete_file)

    return {
        'statusCode': 200,
        'body': json.dumps({
            'flag': flag,
            'files': {
                'cat.jpg': filename
            }
        })
    }


def entrypoint(event, context):
    '''
    Entrypoint for calls from AWS Lambda, which passes in the `event`
    and `context` arguments.
    '''
    return generate()


if __name__ == '__main__':
    print(json.dumps(generate(), indent=2))
