#!/usr/bin/env python

import libs.alphabet
import unittest


class TestHelperMethods(unittest.TestCase):

    # --- generate_flag() tests ---

    def test_generate_flag_is_str(self):
        flag = libs.alphabet.generate_flag()
        self.assertIsInstance(flag, str)

    def test_generate_flag_structure(self):
        flag = libs.alphabet.generate_flag()
        self.assertEqual(len(flag), libs.alphabet.FLAG_LENGTH)

    def test_generate_flag_in_correct_range(self):
        flag = libs.alphabet.generate_flag()
        for char in flag:
            self.assertTrue(char in libs.alphabet.ALPHABET)

    # --- scramble_alphabet() tests ---

    def test_scramble_alphabet_is_list(self):
        new_alphabet = libs.alphabet.scramble_alphabet()
        self.assertIsInstance(new_alphabet, list)

    def test_scramble_alphabet_length(self):
        new_alphabet = libs.alphabet.scramble_alphabet()
        self.assertEqual(len(new_alphabet), len(libs.alphabet.ALPHABET))

    def test_scramble_alphabet_preserves_range(self):
        new_alphabet = libs.alphabet.scramble_alphabet()
        for char in new_alphabet:
            self.assertTrue(char in libs.alphabet.ALPHABET)

    # --- generate_random_shift() tests ---

    def test_generate_random_shift_is_int(self):
        shift = libs.alphabet.generate_random_shift()
        self.assertIsInstance(shift, int)

    def test_generate_random_shift_preserves_range(self):
        shift = libs.alphabet.generate_random_shift()
        self.assertGreaterEqual(shift, 1)
        self.assertLessEqual(shift, len(libs.alphabet.ALPHABET))

    # --- mod_by_alphabet_size() tests ---

    def test_mod_by_alphabet_size_zero(self):
        index = 0
        result_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsInstance(result_index, int)
        self.assertEqual(result_index, index % len(libs.alphabet.ALPHABET))

    def test_mod_by_alphabet_size_in_range(self):
        index = 10
        result_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsInstance(result_index, int)
        self.assertEqual(result_index, index % len(libs.alphabet.ALPHABET))

    def test_mod_by_alphabet_size_out_of_range(self):
        index = 500
        result_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsInstance(result_index, int)
        self.assertEqual(result_index, index % len(libs.alphabet.ALPHABET))

    # --- shift_char() tests ---

    def test_shift_char_zero(self):
        index = 0
        result_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsInstance(result_index, int)
        self.assertEqual(result_index, index % len(libs.alphabet.ALPHABET))

    def test_mod_by_alphabet_size_in_range(self):
        index = 10
        result_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsInstance(result_index, int)
        self.assertEqual(result_index, index % len(libs.alphabet.ALPHABET))

    def test_mod_by_alphabet_size_out_of_range(self):
        index = 500
        result_index = libs.alphabet.mod_by_alphabet_size(index)
        self.assertIsInstance(result_index, int)
        self.assertEqual(result_index, index % len(libs.alphabet.ALPHABET))

if __name__ == '__main__':
    unittest.main()
