import random


ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
FLAG_LENGTH = 16


def generate_flag():
    """
    Generate a random flag.
    """
    flag = ""
    for _ in range(FLAG_LENGTH):
        flag += random.choice(ALPHABET)
    return flag


def scramble_alphabet():
    """
    Construct a new alphabet by scrambling the original alphabet.
    """
    return random.sample(ALPHABET, len(ALPHABET))


def generate_random_shift():
    """
    Generate a random shift value within the alphabet range.
    """
    alphabet_range = range(1, len(ALPHABET))
    return random.choice(alphabet_range)


def mod_by_alphabet_size(index):
    """
    Mod the given index to keep it within the alphabet range.
    """
    return index % len(ALPHABET)


def shift_char(char, shift):
    """
    Shift the given character by the shift value, preserving the alphabet range.
    """
    initial_index = ALPHABET.index(char)
    shifted_index = initial_index + shift
    shifted_index_in_alphabet_range = mod_by_alphabet_size(shifted_index)
    return ALPHABET[shifted_index_in_alphabet_range]
