#!/usr/bin/env python

import libs.alphabet
import unittest


class TestHelperMethods(unittest.TestCase):

    # --- generate_flag() tests ---

    def test_generate_flag(self):
        flag = libs.alphabet.generate_flag()
        self.assertIsNotNone(flag)

    def test_generate_flag_structure(self):
        flag = libs.alphabet.generate_flag()

        self.assertEqual(len(flag), libs.alphabet.FLAG_LENGTH)

    def test_generate_flag_in_correct_range(self):
        flag = libs.alphabet.generate_flag()

        for char in flag:
            self.assertTrue(char in libs.alphabet.ALPHABET)

    # --- scramble_alphabet() tests ---

    def test_scramble_alphabet(self):
        new_alphabet = libs.alphabet.scramble_alphabet()
        self.assertIsNotNone(new_alphabet)

    def test_scramble_alphabet_length(self):
        new_alphabet = libs.alphabet.scramble_alphabet()
        self.assertEqual(len(new_alphabet), len(libs.alphabet.ALPHABET))

    def test_scramble_alphabet_preserves_range(self):
        new_alphabet = libs.alphabet.scramble_alphabet()
        for char in new_alphabet:
            self.assertTrue(char in libs.alphabet.ALPHABET)

    # --- generate_random_shift() tests ---

    def test_generate_random_shift(self):
        shift = libs.alphabet.generate_random_shift()
        self.assertIsNotNone(shift)

    def test_generate_random_shift_preserves_range(self):
        shift = libs.alphabet.generate_random_shift()
        self.assertTrue(shift, range(1, len(libs.alphabet.ALPHABET)))

    # --- ascii_mod() tests ---

    def test_mod_by_alphabet_size_zero(self):
        index = 0
        mod_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsNotNone(mod_index)
        self.assertEqual(mod_index, (index % len(libs.alphabet.ALPHABET)))

    def test_mod_by_alphabet_size_in_range(self):
        index = 10
        mod_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsNotNone(mod_index)
        self.assertEqual(mod_index, (index % len(libs.alphabet.ALPHABET)))

    def test_mod_by_alphabet_size_out_of_range(self):
        index = 500
        mod_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsNotNone(mod_index)
        self.assertEqual(mod_index, (index % len(libs.alphabet.ALPHABET)))

if __name__ == '__main__':
    unittest.main()
