import unittest
import libftd3xx as ftd3xx

class LibraryVersionTestCase(unittest.TestCase):
    def setUp(self):
        pass

    def test_get_library_version(self):
        _ = ftd3xx.get_library_version()

class DriverVersionTestCase(unittest.TestCase):
    def setUp(self):
        pass

    def test_get_driver_version(self):
        self.assertRaises(TypeError, ftd3xx.get_driver_version, (None,))